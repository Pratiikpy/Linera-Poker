//! Hand contract state using Linera views

use linera_poker_shared::{Card, GameResultInfo, Seat};
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
    /// Dealer secret for card reveals
    pub dealer_secret: RegisterView<Vec<u8>>,
}
