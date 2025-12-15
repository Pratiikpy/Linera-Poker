//! Linera Poker - Hand Contract (Player Chain) ABI
//!
//! Stores player's PRIVATE hole cards. Only the chain owner can see them.
//! The dealer chain CANNOT access this state - architectural privacy guarantee.

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{Amount, ApplicationId, ChainId, ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub use linera_poker_shared::{
    BetAction, Card, CardReveal, EncryptedCard, GamePhase, GameResultInfo, Seat,
};

/// Hand contract ABI
pub struct HandAbi;

impl ContractAbi for HandAbi {
    type Operation = HandOperation;
    type Response = HandResult;
}

impl ServiceAbi for HandAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Result of hand operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandResult {
    Success,
    Error(HandError),
}

/// Hand errors
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum HandError {
    #[error("Not registered with a table")]
    NotRegistered,
    #[error("Already in a game")]
    AlreadyInGame,
    #[error("Not your turn")]
    NotYourTurn,
    #[error("Invalid game state")]
    InvalidState,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid message source")]
    InvalidSource,
}

/// Hand operations (called by player on their own chain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandOperation {
    /// Join a table with stake
    JoinTable { stake: u64 },
    /// Send a betting action
    Bet { action: BetAction },
    /// Reveal cards for showdown
    Reveal,
    /// Leave the table
    LeaveTable,
}

/// Instantiation argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiationArgument {
    pub table_chain: ChainId,
    pub table_app: ApplicationId,
}

// Re-export unified Message from shared crate for cross-chain messaging
pub use linera_poker_shared::Message;
