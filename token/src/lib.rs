//! Linera Poker - Token Contract (Player Chain) ABI
//!
//! Chip/token balance for a player. Only the chain OWNER can decrease balance.
//! Demonstrates TRUE Linera token sovereignty - no one can take your chips without permission.

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{Amount, ChainId, ContractAbi, ServiceAbi, AccountOwner};
use serde::{Deserialize, Serialize};

/// Token contract ABI
pub struct TokenAbi;

impl ContractAbi for TokenAbi {
    type Operation = TokenOperation;
    type Response = TokenResult;
}

impl ServiceAbi for TokenAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Result of token operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenResult {
    Success,
    Error(TokenError),
}

/// Token errors
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum TokenError {
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Unauthorized - only owner can decrease balance")]
    Unauthorized,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Already locked")]
    AlreadyLocked,
}

/// Token operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenOperation {
    /// Deposit chips (anyone can increase balance)
    Deposit { amount: u64 },
    /// Withdraw chips (owner only)
    Withdraw { amount: u64 },
    /// Lock stake for a game
    /// FIX #3: Added game_id parameter
    LockForGame { amount: u64, table_chain: ChainId, game_id: u64 },
    /// Transfer to another chain
    /// FIX #3: Added game_id parameter
    Transfer { to_chain: ChainId, amount: u64, game_id: u64 },
}

/// Instantiation argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiationArgument {
    pub owner: AccountOwner,
    pub initial_balance: u64,
}

/// Cross-chain messages for Token contract
/// Contains BOTH incoming (from Table) and outgoing (to Table) message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // ═══════════════════════════════════════════════════════════════════
    // INCOMING from Table chain
    // ═══════════════════════════════════════════════════════════════════

    /// Request player to lock stake for game
    LockStake {
        game_id: u64,
        amount: Amount,
    },
    /// Payout winnings to player
    Payout {
        game_id: u64,
        amount: Amount,
    },
    /// Refund stake (game cancelled)
    Refund {
        game_id: u64,
        amount: Amount,
    },

    // ═══════════════════════════════════════════════════════════════════
    // OUTGOING to Table chain
    // ═══════════════════════════════════════════════════════════════════

    /// Stake has been locked
    StakeLocked {
        game_id: u64,
        amount: Amount,
    },
    /// Stake lock failed (insufficient funds)
    StakeFailed {
        game_id: u64,
        reason: String,
    },
}
