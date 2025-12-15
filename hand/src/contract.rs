#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::HandState;
use linera_poker_hand::{
    BetAction, Card, CardReveal, GamePhase, GameResultInfo, HandAbi,
    HandOperation, HandResult, InstantiationArgument, Message,
};
use linera_sdk::{
    linera_base_types::{Amount, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct HandContract {
    state: HandState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(HandContract);

impl WithContractAbi for HandContract {
    type Abi = HandAbi;
}

impl Contract for HandContract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = InstantiationArgument;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = HandState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        HandContract { state, runtime }
    }

    async fn instantiate(&mut self, arg: InstantiationArgument) {
        self.state.game_id.set(None);
        self.state.table_chain.set(Some(arg.table_chain));
        self.state.table_app.set(Some(arg.table_app));
        self.state.seat.set(None);
        self.state.hole_cards.set(Vec::new());
        self.state.community_cards.set(Vec::new());
        self.state.current_bet.set(Amount::ZERO);
        self.state.my_turn.set(false);
        self.state.game_result.set(None);
        self.state.dealer_secret.set(Vec::new());
    }

    async fn execute_operation(&mut self, operation: HandOperation) -> HandResult {
        match operation {
            HandOperation::JoinTable { stake } => {
                self.join_table(Amount::from_tokens(stake.into())).await
            }
            HandOperation::Bet { action } => {
                self.send_bet_action(action).await
            }
            HandOperation::Reveal => {
                self.reveal_cards().await
            }
            HandOperation::LeaveTable => {
                self.leave_table().await
            }
        }
    }

    async fn execute_message(&mut self, message: Message) {
        let source_chain = match self.runtime.message_origin_chain_id() {
            Some(chain_id) => chain_id,
            None => return,
        };

        // FIX #7: HIGH - Require table_chain to be set before processing messages
        let table_chain = match self.state.table_chain.get() {
            Some(c) => *c,
            None => return, // Must be configured with a table_chain first
        };

        // RELAY PATTERN: If we're running on the table chain, relay player->table messages
        let current_chain = self.runtime.chain_id();
        let is_relay = current_chain == table_chain;

        match message {
            // INCOMING messages from Table chain to player
            Message::DealCards { game_id, encrypted_cards: _ } => {
                // Only process if we're on a player chain (source should be table)
                if source_chain != table_chain {
                    return; // Reject messages from unauthorized chains
                }
                self.state.game_id.set(Some(game_id));
            }
            Message::CommunityCards { game_id, phase, cards } => {
                // Only process if we're on a player chain (source should be table)
                if source_chain != table_chain {
                    return; // Reject messages from unauthorized chains
                }
                self.handle_community_cards(game_id, phase, cards);
            }
            Message::RequestReveal { game_id: _ } => {
                // Only process if we're on a player chain (source should be table)
                if source_chain != table_chain {
                    return; // Reject messages from unauthorized chains
                }
                self.state.my_turn.set(true);
            }
            Message::YourTurn { game_id, current_bet, pot: _, min_raise: _ } => {
                // Only process if we're on a player chain (source should be table)
                if source_chain != table_chain {
                    return; // Reject messages from unauthorized chains
                }
                self.handle_your_turn(game_id, current_bet);
            }
            Message::GameResult { game_id, you_won, payout, opponent_cards } => {
                // Only process if we're on a player chain (source should be table)
                if source_chain != table_chain {
                    return; // Reject messages from unauthorized chains
                }
                self.handle_game_result(game_id, you_won, payout, opponent_cards);
            }

            // RELAY messages from player chains to table app
            // These messages arrive here when sent to table_chain via send_to()
            Message::JoinTable { stake, hand_app_id } => {
                if is_relay {
                    // We're the relay on table chain - forward to table app
                    self.relay_to_table(message).await;
                }
                // If not relay, ignore (player chains don't handle this)
            }
            Message::CardsReceived { game_id: _ } => {
                if is_relay {
                    // We're the relay on table chain - forward to table app
                    self.relay_to_table(message).await;
                }
            }
            Message::BetAction { game_id: _, action: _ } => {
                if is_relay {
                    // We're the relay on table chain - forward to table app
                    self.relay_to_table(message).await;
                }
            }
            Message::RevealCards { game_id: _, cards: _, proofs: _ } => {
                if is_relay {
                    // We're the relay on table chain - forward to table app
                    self.relay_to_table(message).await;
                }
            }
            Message::LeaveTable => {
                if is_relay {
                    // We're the relay on table chain - forward to table app
                    self.relay_to_table(message).await;
                }
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl HandContract {
    /// Relay a message to the table application
    /// This is called when the hand app on the table chain receives a message
    /// that needs to be forwarded to the table app (different application ID)
    async fn relay_to_table(&mut self, message: Message) {
        let table_app = match self.state.table_app.get() {
            Some(app) => *app,
            None => return, // No table app configured - cannot relay
        };

        // Get the source chain of the original message
        let source_chain = match self.runtime.message_origin_chain_id() {
            Some(chain_id) => chain_id,
            None => return,
        };

        // Convert Message to TableOperation for cross-application call
        // We need to import TableOperation from linera_poker_table
        use linera_poker_table::TableOperation;

        let operation = match message {
            Message::JoinTable { stake, hand_app_id } => {
                TableOperation::RelayJoinTable {
                    player_chain: source_chain,
                    stake,
                    hand_app_id,
                }
            }
            Message::BetAction { game_id, action } => {
                TableOperation::RelayBetAction {
                    player_chain: source_chain,
                    game_id,
                    action,
                }
            }
            Message::RevealCards { game_id, cards, proofs } => {
                TableOperation::RelayRevealCards {
                    player_chain: source_chain,
                    game_id,
                    cards,
                    proofs,
                }
            }
            Message::LeaveTable => {
                TableOperation::RelayLeaveTable {
                    player_chain: source_chain,
                }
            }
            Message::CardsReceived { game_id } => {
                TableOperation::RelayCardsReceived {
                    player_chain: source_chain,
                    game_id,
                }
            }
            // Table->Hand messages should not be relayed
            _ => return,
        };

        // Use call_application to invoke the operation on the table app
        // We use authenticated=true to preserve the original message sender's authentication
        let _result = self.runtime
            .call_application(
                /* authenticated */ true,
                table_app.with_abi::<linera_poker_table::TableAbi>(),
                &operation,
            );

        // Note: We ignore the result here. In a production system, you might want to:
        // 1. Log errors for debugging
        // 2. Send error responses back to the source chain
        // 3. Implement retry logic for transient failures
        // For now, we simply forward the operation and let the table app handle it
    }

    /// Join a table
    async fn join_table(&mut self, stake: Amount) -> HandResult {
        if self.state.game_id.get().is_some() {
            return HandResult::Error(linera_poker_hand::HandError::AlreadyInGame);
        }

        let table_chain = match self.state.table_chain.get() {
            Some(c) => *c,
            None => return HandResult::Error(linera_poker_hand::HandError::NotRegistered),
        };

        let our_app_id = self.runtime.application_id();

        self.runtime
            .prepare_message(Message::JoinTable {
                stake,
                hand_app_id: our_app_id.forget_abi(),
            })
            .with_authentication()
            .send_to(table_chain);

        HandResult::Success
    }

    /// Handle receiving cards
    fn handle_community_cards(
        &mut self,
        game_id: u64,
        phase: GamePhase,
        cards: Vec<CardReveal>,
    ) {
        if self.state.game_id.get() != &Some(game_id) && self.state.game_id.get().is_some() {
            return;
        }

        self.state.game_id.set(Some(game_id));

        if phase == GamePhase::Dealing {
            // These are our hole cards!
            let hole_cards: Vec<Card> = cards.iter().map(|r| r.card).collect();
            self.state.hole_cards.set(hole_cards);
            // Store the dealer secret from first card reveal
            if let Some(first) = cards.first() {
                self.state.dealer_secret.set(first.secret.clone());
            }
        } else {
            // Community cards
            let mut community = self.state.community_cards.get().clone();
            for reveal in cards {
                if !community.contains(&reveal.card) {
                    community.push(reveal.card);
                }
            }
            self.state.community_cards.set(community);
        }
    }

    /// Handle it's our turn
    fn handle_your_turn(&mut self, game_id: u64, current_bet: Amount) {
        if self.state.game_id.get() != &Some(game_id) {
            return;
        }

        self.state.my_turn.set(true);
        self.state.current_bet.set(current_bet);
    }

    /// Send betting action
    async fn send_bet_action(&mut self, action: BetAction) -> HandResult {
        if !*self.state.my_turn.get() {
            return HandResult::Error(linera_poker_hand::HandError::NotYourTurn);
        }

        let game_id = match self.state.game_id.get() {
            Some(id) => *id,
            None => return HandResult::Error(linera_poker_hand::HandError::InvalidState),
        };

        let table_chain = match self.state.table_chain.get() {
            Some(c) => *c,
            None => return HandResult::Error(linera_poker_hand::HandError::NotRegistered),
        };

        self.runtime
            .prepare_message(Message::BetAction { game_id, action })
            .with_authentication()
            .send_to(table_chain);

        self.state.my_turn.set(false);

        HandResult::Success
    }

    /// Reveal our cards
    async fn reveal_cards(&mut self) -> HandResult {
        let game_id = match self.state.game_id.get() {
            Some(id) => *id,
            None => return HandResult::Error(linera_poker_hand::HandError::InvalidState),
        };

        let table_chain = match self.state.table_chain.get() {
            Some(c) => *c,
            None => return HandResult::Error(linera_poker_hand::HandError::NotRegistered),
        };

        let cards = self.state.hole_cards.get().clone();
        let dealer_secret = self.state.dealer_secret.get().clone();
        let proofs: Vec<CardReveal> = cards
            .iter()
            .map(|card| CardReveal {
                card: *card,
                secret: dealer_secret.clone(),
            })
            .collect();

        self.runtime
            .prepare_message(Message::RevealCards { game_id, cards, proofs })
            .with_authentication()
            .send_to(table_chain);

        self.state.my_turn.set(false);

        HandResult::Success
    }

    /// Handle game result
    fn handle_game_result(
        &mut self,
        game_id: u64,
        won: bool,
        payout: Amount,
        opponent_cards: Option<Vec<Card>>,
    ) {
        if self.state.game_id.get() != &Some(game_id) {
            return;
        }

        self.state.game_result.set(Some(GameResultInfo {
            won,
            payout,
            my_cards: self.state.hole_cards.get().clone(),
            opponent_cards,
        }));

        self.state.my_turn.set(false);
    }

    /// Leave the table
    async fn leave_table(&mut self) -> HandResult {
        let table_chain = match self.state.table_chain.get() {
            Some(c) => *c,
            None => return HandResult::Error(linera_poker_hand::HandError::NotRegistered),
        };

        self.runtime
            .prepare_message(Message::LeaveTable)
            .with_authentication()
            .send_to(table_chain);

        // Reset state
        self.state.game_id.set(None);
        self.state.hole_cards.set(Vec::new());
        self.state.community_cards.set(Vec::new());
        self.state.my_turn.set(false);
        self.state.game_result.set(None);

        HandResult::Success
    }
}
