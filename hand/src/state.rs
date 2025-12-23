//! Hand contract state using Linera views

use linera_poker_shared::{Card, CardCommitment, GameResultInfo, Seat};
use linera_sdk::{
    linera_base_types::{Amount, ApplicationId, ChainId},
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};

/// Hand state stored on player's chain (PRIVATE)
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct HandState {
    /// Current game ID
    pub game_id: RegisterView<Option<u64>>,
    /// Table chain we're playing at
    pub table_chain: RegisterView<Option<ChainId>>,
    /// Table application ID
    pub table_app: RegisterView<Option<ApplicationId>>,
    /// Our seat at the table
    pub seat: RegisterView<Option<Seat>>,
    /// Our hole cards (PRIVATE - only visible on this chain)
    pub hole_cards: RegisterView<Vec<Card>>,
    /// Community cards we've received
    pub community_cards: RegisterView<Vec<Card>>,
    /// Current bet to match
    pub current_bet: RegisterView<Amount>,
    /// Is it our turn?
    pub my_turn: RegisterView<bool>,
    /// Game result (if game ended)
    pub game_result: RegisterView<Option<GameResultInfo>>,

    // ========================================================================
    // DEPRECATED: INSECURE FIELDS (Phase 3: Marked for Removal)
    // ========================================================================

    /// DEPRECATED: Dealer secret for card reveals
    /// Phase 3: Replaced by ZK commitments - keep for backward compatibility
    pub dealer_secret: RegisterView<Vec<u8>>,

    // ========================================================================
    // ZK-SNARK STATE (Phase 3: Production-Ready Privacy)
    // ========================================================================

    /// Card commitments received from table (for reveal proof generation)
    pub card_commitments: RegisterView<Option<Vec<CardCommitment>>>,

    /// Blinding factors used in commitments (for reveal proof)
    /// These are sent by the table along with the DealingProof
    pub blinding_factors: RegisterView<Option<Vec<Vec<u8>>>>,

    /// Deck root from table (for verification)
    pub table_deck_root: RegisterView<Option<[u8; 32]>>,

    /// Turn deadline block (for timeout awareness)
    pub turn_deadline_block: RegisterView<Option<u64>>,
}
