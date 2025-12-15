#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TokenState;
use linera_poker_token::{
    InstantiationArgument, Message, TokenAbi, TokenError,
    TokenOperation, TokenResult,
};
use linera_sdk::{
    linera_base_types::{Amount, ChainId, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct TokenContract {
    state: TokenState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(TokenContract);

impl WithContractAbi for TokenContract {
    type Abi = TokenAbi;
}

impl Contract for TokenContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = InstantiationArgument;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TokenContract { state, runtime }
    }

    async fn instantiate(&mut self, arg: InstantiationArgument) {
        self.state.balance.set(Amount::from_tokens(arg.initial_balance.into()));
        self.state.locked.set(Amount::ZERO);
        self.state.owner.set(Some(arg.owner));
    }

    async fn execute_operation(&mut self, operation: TokenOperation) -> TokenResult {
        match operation {
            TokenOperation::Deposit { amount } => {
                self.deposit(Amount::from_tokens(amount.into()))
            }
            TokenOperation::Withdraw { amount } => {
                self.withdraw(Amount::from_tokens(amount.into()))
            }
            TokenOperation::LockForGame { amount, table_chain, game_id } => {
                self.lock_stake(Amount::from_tokens(amount.into()), table_chain, game_id).await
            }
            TokenOperation::Transfer { to_chain, amount, game_id } => {
                self.transfer(to_chain, Amount::from_tokens(amount.into()), game_id).await
            }
        }
    }

    async fn execute_message(&mut self, message: Message) {
        match message {
            // INCOMING messages from Table chain
            Message::LockStake { game_id: _, amount: _ } => {
                // Acknowledgment - stake already locked via operation
            }
            Message::Payout { game_id: _, amount } => {
                self.receive_payout(amount);
            }
            Message::Refund { game_id: _, amount } => {
                self.unlock_stake(amount);
            }
            // OUTGOING messages (shouldn't be received)
            _ => {}
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl TokenContract {
    /// Deposit chips
    fn deposit(&mut self, amount: Amount) -> TokenResult {
        if amount == Amount::ZERO {
            return TokenResult::Error(TokenError::InvalidAmount);
        }
        let balance = *self.state.balance.get();
        self.state.balance.set(balance.saturating_add(amount));
        TokenResult::Success
    }

    /// Withdraw chips
    fn withdraw(&mut self, amount: Amount) -> TokenResult {
        let balance = *self.state.balance.get();
        let locked = *self.state.locked.get();
        let available = balance.saturating_sub(locked);

        if amount > available {
            return TokenResult::Error(TokenError::InsufficientBalance);
        }

        self.state.balance.set(balance.saturating_sub(amount));
        TokenResult::Success
    }

    /// Lock stake for a game
    /// FIX #3: CRITICAL - Accept and use actual game_id instead of hardcoded 0
    async fn lock_stake(&mut self, amount: Amount, table_chain: ChainId, game_id: u64) -> TokenResult {
        let balance = *self.state.balance.get();
        let locked = *self.state.locked.get();
        let available = balance.saturating_sub(locked);

        if amount > available {
            return TokenResult::Error(TokenError::InsufficientBalance);
        }

        self.state.locked.set(locked.saturating_add(amount));

        // FIX #3: Use actual game_id parameter
        self.runtime
            .prepare_message(Message::StakeLocked {
                game_id,
                amount,
            })
            .with_authentication()
            .send_to(table_chain);

        TokenResult::Success
    }

    /// Receive payout
    fn receive_payout(&mut self, amount: Amount) {
        self.state.locked.set(Amount::ZERO);
        let balance = *self.state.balance.get();
        self.state.balance.set(balance.saturating_add(amount));
    }

    /// Unlock stake (refund)
    fn unlock_stake(&mut self, amount: Amount) {
        let locked = *self.state.locked.get();
        self.state.locked.set(locked.saturating_sub(amount));
    }

    /// Transfer to another chain
    /// FIX #3: CRITICAL - Accept and use actual game_id instead of hardcoded 0
    async fn transfer(&mut self, to_chain: ChainId, amount: Amount, game_id: u64) -> TokenResult {
        let balance = *self.state.balance.get();
        let locked = *self.state.locked.get();
        let available = balance.saturating_sub(locked);

        if amount > available {
            return TokenResult::Error(TokenError::InsufficientBalance);
        }

        self.state.balance.set(balance.saturating_sub(amount));

        // FIX #3: Use actual game_id parameter
        self.runtime
            .prepare_message(Message::Payout {
                game_id,
                amount,
            })
            .with_authentication()
            .send_to(to_chain);

        TokenResult::Success
    }
}
