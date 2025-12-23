//! Table contract state using Linera views

use linera_poker_shared::{Card, CardCommitment, GamePhase, PlayerInfo, RevealProof, Seat};
use linera_sdk::{
    linera_base_types::{Amount, ChainId},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// TIMEOUT CONFIGURATION (Phase 3: Liveness Guarantees)
// ============================================================================

/// Timeout configuration for auto-forfeit mechanics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Blocks until bet action times out (default: 50 blocks ~ 5 minutes)
    pub bet_timeout_blocks: u32,
    /// Blocks until reveal times out (default: 100 blocks ~ 10 minutes)
    pub reveal_timeout_blocks: u32,
    /// Whether auto-forfeit is enabled
    pub auto_forfeit_enabled: bool,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            bet_timeout_blocks: 50,      // ~5 minutes at 6 seconds/block
            reveal_timeout_blocks: 100,   // ~10 minutes
            auto_forfeit_enabled: true,
        }
    }
}

// ============================================================================
// ZK PROOF PARAMETERS (Phase 3: Production-Ready Privacy)
// ============================================================================

/// Proof parameters for ZK-SNARKs (paths to verifying keys)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokerProofParams {
    /// Path to dealing circuit verifying key
    pub dealing_vk_path: String,
    /// Path to reveal circuit verifying key
    pub reveal_vk_path: String,
}

impl Default for PokerProofParams {
    fn default() -> Self {
        Self {
            dealing_vk_path: "keys/dealing.vk".to_string(),
            reveal_vk_path: "keys/reveal.vk".to_string(),
        }
    }
}

// ============================================================================
// TABLE STATE (Phase 3: ZK-Enhanced)
// ============================================================================

/// Table state stored on-chain using views
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct TableState {
    // ========================================================================
    // CORE GAME STATE (Existing)
    // ========================================================================
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

    // ========================================================================
    // TABLE CONFIGURATION (Existing)
    // ========================================================================
    /// Minimum stake to join
    pub min_stake: RegisterView<Amount>,
    /// Maximum stake to join
    pub max_stake: RegisterView<Amount>,
    /// Small blind amount (button posts this)
    pub small_blind: RegisterView<Amount>,
    /// Big blind amount (non-button posts this)
    pub big_blind: RegisterView<Amount>,
    /// Current dealer button position (alternates each hand)
    pub dealer_button: RegisterView<Option<Seat>>,

    // ========================================================================
    // DEPRECATED: INSECURE FIELDS (Phase 3: Removed)
    // ========================================================================
    // REMOVED: pub dealer_secret: RegisterView<Vec<u8>>
    // ^^^ SECURITY ISSUE: This exposed secret to GraphQL queries!
    // ^^^ Replaced by ZK commitments below

    /// Deck seed (for deterministic shuffle)
    /// NOTE: Still deterministic - will be replaced by commit-reveal in future
    pub deck_seed: RegisterView<Vec<u8>>,

    // ========================================================================
    // ZK-SNARK STATE (Phase 3: Production-Ready Privacy)
    // ========================================================================
    /// ZK proof parameters (verifying key paths)
    pub proof_params: RegisterView<PokerProofParams>,

    /// Merkle root of shuffled deck (for proving card inclusion)
    pub deck_root: RegisterView<[u8; 32]>,

    /// Player card commitments (Pedersen commitments)
    /// Maps ChainId -> [hole_card_1_commitment, hole_card_2_commitment]
    pub player_commitments: MapView<ChainId, Vec<CardCommitment>>,

    /// Revealed hole cards with ZK proofs (for showdown)
    /// Stores RevealProof instead of plaintext cards
    pub revealed_cards_zk: RegisterView<Vec<(Seat, RevealProof)>>,

    // ========================================================================
    // LEGACY REVEALED CARDS (Backward Compatibility)
    // ========================================================================
    /// DEPRECATED: Revealed hole cards from players (plaintext)
    /// Use revealed_cards_zk instead for production
    pub revealed_cards: RegisterView<Vec<(Seat, Vec<Card>)>>,

    // ========================================================================
    // TIMEOUT & LIVENESS (Phase 3: Anti-Griefing)
    // ========================================================================
    /// Timeout configuration
    pub timeout_config: RegisterView<TimeoutConfig>,

    /// Block height when current turn started (for timeout detection)
    pub turn_start_block: RegisterView<u64>,

    /// Block height when showdown phase started (for reveal timeout)
    pub showdown_start_block: RegisterView<Option<u64>>,

    /// Players who have timed out (auto-forfeited)
    pub timed_out_players: RegisterView<Vec<ChainId>>,

    // ========================================================================
    // BETTING ROUND STATE (Existing)
    // ========================================================================
    /// Number of actions taken in current betting round (to prevent premature phase advance)
    pub actions_this_round: RegisterView<u8>,
}
