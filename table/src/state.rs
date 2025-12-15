//! Table contract state using Linera views

use linera_sdk::{
    linera_base_types::Amount,
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};
use linera_poker_shared::{Card, GamePhase, PlayerInfo, Seat};

/// Table state stored on-chain using views
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct TableState {
    /// Current game ID
    pub game_id: RegisterView<u64>,
    /// Current game phase
    pub phase: RegisterView<GamePhase>,
    /// Players (max 2)
    pub players: RegisterView<Vec<PlayerInfo>>,
    /// Total pot
    pub pot: RegisterView<Amount>,
    /// Current bet to call
    pub current_bet: RegisterView<Amount>,
    /// Minimum raise amount
    pub min_raise: RegisterView<Amount>,
    /// Community cards (flop/turn/river)
    pub community_cards: RegisterView<Vec<Card>>,
    /// Whose turn it is
    pub turn_seat: RegisterView<Option<Seat>>,
    /// Winner (if determined)
    pub winner: RegisterView<Option<Seat>>,
    /// Minimum stake to join
    pub min_stake: RegisterView<Amount>,
    /// Maximum stake to join
    pub max_stake: RegisterView<Amount>,
    /// Deck seed (for deterministic shuffle)
    pub deck_seed: RegisterView<Vec<u8>>,
    /// Dealer secret (for card commitments)
    pub dealer_secret: RegisterView<Vec<u8>>,
    /// Revealed hole cards from players (for showdown)
    pub revealed_cards: RegisterView<Vec<(Seat, Vec<Card>)>>,
    /// Number of actions taken in current betting round (to prevent premature phase advance)
    pub actions_this_round: RegisterView<u8>,
    /// Small blind amount (button posts this)
    pub small_blind: RegisterView<Amount>,
    /// Big blind amount (non-button posts this)
    pub big_blind: RegisterView<Amount>,
    /// Current dealer button position (alternates each hand)
    pub dealer_button: RegisterView<Option<Seat>>,
}
