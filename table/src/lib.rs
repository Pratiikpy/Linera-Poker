//! Linera Poker - Table Contract (Dealer Chain) ABI
//!
//! Manages game lifecycle, pot, deck commitment, and settlement.
//! This contract CANNOT see player hole cards - they are on player chains.

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{Amount, ApplicationId, ChainId, ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub use linera_poker_shared::{
    BetAction, Card, CardReveal, EncryptedCard, GamePhase,
    PlayerInfo, Seat, TableState,
};

/// Table contract ABI
pub struct TableAbi;

impl ContractAbi for TableAbi {
    type Operation = TableOperation;
    type Response = TableResult;
}

impl ServiceAbi for TableAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Result of table operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableResult {
    Success,
    Error(TableError),
}

/// Table errors
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum TableError {
    #[error("Game is full (2 players max)")]
    GameFull,
    #[error("Invalid game phase for this action")]
    InvalidPhase,
    #[error("Not your turn")]
    NotYourTurn,
    #[error("Invalid bet amount")]
    InvalidBet,
    #[error("Player not found")]
    PlayerNotFound,
    #[error("Already joined this table")]
    AlreadyJoined,
    #[error("Insufficient stake")]
    InsufficientStake,
    #[error("Invalid card reveal")]
    InvalidReveal,
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Table operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableOperation {
    /// Start a new game (reset table)
    StartNewGame,
    /// Force advance phase (testing only)
    ForceAdvance,

    // Player actions (relayed from hand app on table chain)
    /// Player joins table with stake (relayed message)
    RelayJoinTable {
        player_chain: ChainId,
        stake: Amount,
        hand_app_id: ApplicationId,
    },
    /// Player's betting action (relayed message)
    RelayBetAction {
        player_chain: ChainId,
        game_id: u64,
        action: BetAction,
    },
    /// Player reveals cards (relayed message)
    RelayRevealCards {
        player_chain: ChainId,
        game_id: u64,
        cards: Vec<Card>,
        proofs: Vec<CardReveal>,
    },
    /// Player leaves table (relayed message)
    RelayLeaveTable {
        player_chain: ChainId,
    },
    /// Player acknowledges cards received (relayed message)
    RelayCardsReceived {
        player_chain: ChainId,
        game_id: u64,
    },
}

/// Instantiation argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiationArgument {
    pub min_stake: u64,
    pub max_stake: u64,
    /// Small blind amount (button posts this)
    pub small_blind: u64,
    /// Big blind amount (non-button posts this)
    pub big_blind: u64,
}

// Re-export unified Message from shared crate for cross-chain messaging
pub use linera_poker_shared::Message;
