//! Linera Poker - Shared Types
//!
//! Cross-Chain Mental Poker Protocol
//! Each player's cards are on their OWN chain - dealer cannot see them.

use async_graphql::{Enum, SimpleObject};
use linera_sdk::linera_base_types::{ApplicationId, ChainId, Amount, AccountOwner};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

// ============================================================================
// CARD REPRESENTATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Enum)]
#[repr(u8)]
pub enum Suit {
    Hearts = 0,
    Diamonds = 1,
    Clubs = 2,
    Spades = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Enum)]
#[repr(u8)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, SimpleObject)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    /// Convert card to unique index 0-51
    pub fn to_index(&self) -> u8 {
        (self.suit as u8) * 13 + (self.rank as u8 - 2)
    }

    /// Create card from index 0-51
    pub fn from_index(idx: u8) -> Option<Self> {
        if idx >= 52 {
            return None;
        }
        let suit = match idx / 13 {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            _ => Suit::Spades,
        };
        let rank = match (idx % 13) + 2 {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace,
        };
        Some(Card { suit, rank })
    }
}

// ============================================================================
// ENCRYPTED CARD (Mental Poker Commitment)
// ============================================================================

/// A card encrypted with the dealer's secret key
/// Player cannot know what card this is until they receive the decryption key
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedCard {
    /// SHA256(card_index || dealer_secret || nonce)
    pub commitment: [u8; 32],
    /// Unique nonce for this card
    pub nonce: [u8; 16],
}

impl EncryptedCard {
    pub fn new(card: Card, dealer_secret: &[u8], nonce: [u8; 16]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update([card.to_index()]);
        hasher.update(dealer_secret);
        hasher.update(nonce);
        let commitment: [u8; 32] = hasher.finalize().into();
        Self { commitment, nonce }
    }

    /// Verify that a revealed card matches this commitment
    pub fn verify(&self, card: Card, dealer_secret: &[u8]) -> bool {
        let mut hasher = Sha256::new();
        hasher.update([card.to_index()]);
        hasher.update(dealer_secret);
        hasher.update(self.nonce);
        let expected: [u8; 32] = hasher.finalize().into();
        self.commitment == expected
    }
}

// ============================================================================
// CARD REVEAL (Proof that card matches commitment)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardReveal {
    /// The actual card
    pub card: Card,
    /// The dealer's secret used in commitment
    pub secret: Vec<u8>,
}

// ============================================================================
// PLAYER SEAT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, Enum)]
pub enum Seat {
    Player1,
    Player2,
}

impl Seat {
    pub fn other(&self) -> Self {
        match self {
            Seat::Player1 => Seat::Player2,
            Seat::Player2 => Seat::Player1,
        }
    }
}

// ============================================================================
// PLAYER INFO
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub seat: Seat,
    pub chain_id: ChainId,
    pub owner: AccountOwner,
    pub stake: Amount,
    pub hand_app: Option<ApplicationId>,
    pub has_folded: bool,
    pub current_bet: Amount,
    pub has_revealed: bool,
}

// ============================================================================
// BETTING ACTIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BetAction {
    /// Match the current bet
    Check,
    /// Match the current bet (when there's a raise to match)
    Call,
    /// Increase the bet
    Raise(Amount),
    /// Go all-in
    AllIn,
    /// Give up the hand
    Fold,
}

// ============================================================================
// GAME PHASES (State Machine)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Enum)]
pub enum GamePhase {
    /// Waiting for players to join
    #[default]
    WaitingForPlayers,
    /// Players have joined, dealing cards
    Dealing,
    /// Pre-flop betting round
    PreFlop,
    /// Flop dealt, betting
    Flop,
    /// Turn dealt, betting
    Turn,
    /// River dealt, betting
    River,
    /// All betting complete, waiting for reveals
    Showdown,
    /// Winner determined, paying out
    Settlement,
    /// Game complete
    Finished,
}

// ============================================================================
// HAND RANKINGS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandRank {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandScore {
    pub rank: HandRank,
    /// Tiebreaker values (e.g., kickers)
    pub tiebreakers: Vec<u8>,
}

impl PartialOrd for HandScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => self.tiebreakers.cmp(&other.tiebreakers),
            ord => ord,
        }
    }
}

// ============================================================================
// CROSS-CHAIN MESSAGES: Table -> Hand
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableToHandMessage {
    /// Dealer sends encrypted hole cards to player
    DealCards {
        game_id: u64,
        encrypted_cards: Vec<EncryptedCard>,
    },
    /// Dealer sends community cards (with reveal keys)
    CommunityCards {
        game_id: u64,
        phase: GamePhase,
        cards: Vec<CardReveal>,
    },
    /// Request player to reveal their cards for showdown
    RequestReveal {
        game_id: u64,
    },
    /// Notify player it's their turn to act
    YourTurn {
        game_id: u64,
        current_bet: Amount,
        pot: Amount,
        min_raise: Amount,
    },
    /// Game result notification
    GameResult {
        game_id: u64,
        you_won: bool,
        payout: Amount,
        opponent_cards: Option<Vec<Card>>,
    },
}

// ============================================================================
// CROSS-CHAIN MESSAGES: Hand -> Table
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandToTableMessage {
    /// Player joins table with stake
    JoinTable {
        stake: Amount,
        hand_app_id: ApplicationId,
    },
    /// Player acknowledges receiving cards
    CardsReceived {
        game_id: u64,
    },
    /// Player's betting action
    BetAction {
        game_id: u64,
        action: BetAction,
    },
    /// Player reveals their hole cards for showdown
    RevealCards {
        game_id: u64,
        cards: Vec<Card>,
        proofs: Vec<CardReveal>,
    },
    /// Player leaves table
    LeaveTable,
}

// ============================================================================
// CROSS-CHAIN MESSAGES: Table -> Token
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableToTokenMessage {
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
}

// ============================================================================
// CROSS-CHAIN MESSAGES: Token -> Table
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenToTableMessage {
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

// ============================================================================
// UNIFIED CROSS-CHAIN MESSAGE TYPE
// ============================================================================
// CRITICAL: Both hand and table contracts MUST use this same Message enum
// to ensure correct serialization/deserialization of cross-chain messages.
// The variant ORDER matters for serde - DO NOT reorder variants!

/// Cross-chain messages between Hand and Table contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    // ═══════════════════════════════════════════════════════════════════
    // Table → Hand messages (indices 0-4)
    // ═══════════════════════════════════════════════════════════════════

    /// Dealer sends encrypted hole cards to player
    DealCards {
        game_id: u64,
        encrypted_cards: Vec<EncryptedCard>,
    },
    /// Dealer sends community cards (with reveal keys)
    CommunityCards {
        game_id: u64,
        phase: GamePhase,
        cards: Vec<CardReveal>,
    },
    /// Request player to reveal their cards for showdown
    RequestReveal {
        game_id: u64,
    },
    /// Notify player it's their turn to act
    YourTurn {
        game_id: u64,
        current_bet: Amount,
        pot: Amount,
        min_raise: Amount,
    },
    /// Game result notification
    GameResult {
        game_id: u64,
        you_won: bool,
        payout: Amount,
        opponent_cards: Option<Vec<Card>>,
    },

    // ═══════════════════════════════════════════════════════════════════
    // Hand → Table messages (indices 5-9)
    // ═══════════════════════════════════════════════════════════════════

    /// Player joins table with stake
    JoinTable {
        stake: Amount,
        hand_app_id: ApplicationId,
    },
    /// Player acknowledges receiving cards
    CardsReceived {
        game_id: u64,
    },
    /// Player's betting action
    BetAction {
        game_id: u64,
        action: BetAction,
    },
    /// Player reveals their hole cards for showdown
    RevealCards {
        game_id: u64,
        cards: Vec<Card>,
        proofs: Vec<CardReveal>,
    },
    /// Player leaves table
    LeaveTable,
}

// ============================================================================
// TABLE CONTRACT STATE (exposed via GraphQL)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TableState {
    pub game_id: u64,
    pub phase: GamePhase,
    pub players: Vec<PlayerInfo>,
    pub pot: Amount,
    pub current_bet: Amount,
    pub min_raise: Amount,
    pub community_cards: Vec<Card>,
    pub turn_seat: Option<Seat>,
    pub winner: Option<Seat>,
    pub min_stake: Amount,
    pub max_stake: Amount,
}

// ============================================================================
// HAND CONTRACT STATE (exposed via GraphQL)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandState {
    pub game_id: Option<u64>,
    pub table_chain: Option<ChainId>,
    pub table_app: Option<ApplicationId>,
    pub seat: Option<Seat>,
    pub hole_cards: Vec<Card>,
    pub community_cards: Vec<Card>,
    pub current_bet: Amount,
    pub my_turn: bool,
    pub game_result: Option<GameResultInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResultInfo {
    pub won: bool,
    pub payout: Amount,
    pub my_cards: Vec<Card>,
    pub opponent_cards: Option<Vec<Card>>,
}

// ============================================================================
// TOKEN CONTRACT STATE (exposed via GraphQL)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenState {
    pub balance: Amount,
    pub locked: Amount,
    pub owner: Option<AccountOwner>,
}

// ============================================================================
// HAND EVALUATION (for determining winner)
// ============================================================================

pub fn evaluate_hand(hole_cards: &[Card], community: &[Card]) -> HandScore {
    let mut all_cards: Vec<Card> = hole_cards.to_vec();
    all_cards.extend(community.iter().cloned());

    // Find best 5-card hand from 7 cards
    let mut best_score = HandScore {
        rank: HandRank::HighCard,
        tiebreakers: vec![],
    };

    // Generate all 5-card combinations from 7 cards
    for combo in combinations(&all_cards, 5) {
        let score = evaluate_five_cards(&combo);
        if score > best_score {
            best_score = score;
        }
    }

    best_score
}

fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.len() < k {
        return vec![];
    }

    let mut result = vec![];
    for i in 0..=items.len() - k {
        let head = items[i].clone();
        for mut tail in combinations(&items[i + 1..], k - 1) {
            tail.insert(0, head.clone());
            result.push(tail);
        }
    }
    result
}

fn evaluate_five_cards(cards: &[Card]) -> HandScore {
    let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();
    ranks.sort_by(|a, b| b.cmp(a)); // Descending

    let suits: Vec<Suit> = cards.iter().map(|c| c.suit).collect();
    let is_flush = suits.iter().all(|s| *s == suits[0]);

    // Check for straight
    let mut sorted_ranks = ranks.clone();
    sorted_ranks.dedup();
    let is_straight = sorted_ranks.len() == 5 &&
        (sorted_ranks[0] - sorted_ranks[4] == 4 ||
         // Ace-low straight (A-2-3-4-5)
         (sorted_ranks == vec![14, 5, 4, 3, 2]));

    // Count rank occurrences
    let mut rank_counts: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();
    for r in &ranks {
        *rank_counts.entry(*r).or_insert(0) += 1;
    }

    let mut counts: Vec<(u8, u8)> = rank_counts.into_iter().collect();
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    let rank_groups: Vec<u8> = counts.iter().map(|(_, count)| *count).collect();

    // Determine hand rank
    let (hand_rank, tiebreakers) = if is_flush && is_straight && ranks[0] == 14 && ranks[1] == 13 {
        (HandRank::RoyalFlush, vec![])
    } else if is_flush && is_straight {
        (HandRank::StraightFlush, vec![ranks[0]])
    } else if rank_groups == vec![4, 1] {
        (HandRank::FourOfAKind, vec![counts[0].0, counts[1].0])
    } else if rank_groups == vec![3, 2] {
        (HandRank::FullHouse, vec![counts[0].0, counts[1].0])
    } else if is_flush {
        (HandRank::Flush, ranks.clone())
    } else if is_straight {
        // Handle ace-low straight
        if ranks == vec![14, 5, 4, 3, 2] {
            (HandRank::Straight, vec![5])
        } else {
            (HandRank::Straight, vec![ranks[0]])
        }
    } else if rank_groups == vec![3, 1, 1] {
        (HandRank::ThreeOfAKind, vec![counts[0].0, counts[1].0, counts[2].0])
    } else if rank_groups == vec![2, 2, 1] {
        (HandRank::TwoPair, vec![counts[0].0, counts[1].0, counts[2].0])
    } else if rank_groups == vec![2, 1, 1, 1] {
        (HandRank::OnePair, vec![counts[0].0, counts[1].0, counts[2].0, counts[3].0])
    } else {
        (HandRank::HighCard, ranks)
    };

    HandScore {
        rank: hand_rank,
        tiebreakers,
    }
}

// ============================================================================
// UTILITY: Generate deterministic "random" deck from seed
// ============================================================================

pub fn shuffle_deck(seed: &[u8]) -> Vec<Card> {
    let mut cards: Vec<Card> = (0..52).filter_map(Card::from_index).collect();

    // Fisher-Yates shuffle using seed
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let hash_bytes: [u8; 32] = hasher.finalize().into();

    for i in (1..52).rev() {
        let j = (hash_bytes[i % 32] as usize) % (i + 1);
        cards.swap(i, j);
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_index_roundtrip() {
        for i in 0..52 {
            let card = Card::from_index(i).unwrap();
            assert_eq!(card.to_index(), i);
        }
    }

    #[test]
    fn test_encrypted_card_verify() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        let secret = b"dealer_secret_key";
        let nonce = [0u8; 16];

        let encrypted = EncryptedCard::new(card, secret, nonce);
        assert!(encrypted.verify(card, secret));

        // Wrong card should fail
        let wrong_card = Card::new(Suit::Spades, Rank::King);
        assert!(!encrypted.verify(wrong_card, secret));
    }

    // FIX #10: MEDIUM - Comprehensive hand evaluation tests

    #[test]
    fn test_royal_flush() {
        let hole = vec![
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community = vec![
            Card::new(Suit::Spades, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Hearts, Rank::Two),
            Card::new(Suit::Clubs, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Eight),
        ];
        let community = vec![
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Hearts, Rank::Six),
            Card::new(Suit::Hearts, Rank::Five),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::StraightFlush);
        assert_eq!(score.tiebreakers[0], 9); // 9-high straight flush
    }

    #[test]
    fn test_four_of_a_kind() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ace),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::FullHouse);
    }

    #[test]
    fn test_flush() {
        let hole = vec![
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Clubs, Rank::Jack),
        ];
        let community = vec![
            Card::new(Suit::Clubs, Rank::Nine),
            Card::new(Suit::Clubs, Rank::Six),
            Card::new(Suit::Clubs, Rank::Three),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::Flush);
    }

    #[test]
    fn test_straight() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Clubs, Rank::Eight),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Seven),
            Card::new(Suit::Spades, Rank::Six),
            Card::new(Suit::Hearts, Rank::Five),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Diamonds, Rank::Two),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::Straight);
        assert_eq!(score.tiebreakers[0], 9); // 9-high straight
    }

    #[test]
    fn test_straight_ace_low() {
        // Ace-low straight (A-2-3-4-5) - also known as "wheel"
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Clubs, Rank::Two),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Three),
            Card::new(Suit::Spades, Rank::Four),
            Card::new(Suit::Hearts, Rank::Five),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Diamonds, Rank::Queen),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::Straight);
        assert_eq!(score.tiebreakers[0], 5); // 5-high (ace-low) straight
    }

    #[test]
    fn test_three_of_a_kind() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Jack),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Jack),
            Card::new(Suit::Clubs, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_two_pair() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::TwoPair);
    }

    #[test]
    fn test_one_pair() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Queen),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::OnePair);
    }

    #[test]
    fn test_high_card() {
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Seven),
            Card::new(Suit::Clubs, Rank::Four),
            Card::new(Suit::Diamonds, Rank::Two),
        ];
        let score = evaluate_hand(&hole, &community);
        assert_eq!(score.rank, HandRank::HighCard);
    }

    #[test]
    fn test_tiebreaker_higher_flush() {
        // Both players have flush, but different high cards
        let hole1 = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Jack),
        ];
        let hole2 = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
        ];
        let community = vec![
            Card::new(Suit::Hearts, Rank::Nine),
            Card::new(Suit::Hearts, Rank::Six),
            Card::new(Suit::Hearts, Rank::Three),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Four),
        ];
        let score1 = evaluate_hand(&hole1, &community);
        let score2 = evaluate_hand(&hole2, &community);

        assert_eq!(score1.rank, HandRank::Flush);
        assert_eq!(score2.rank, HandRank::Flush);
        assert!(score1 > score2); // Ace-high flush beats King-high flush
    }

    #[test]
    fn test_tiebreaker_kicker() {
        // Both have pair of Kings, different kickers
        let hole1 = vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Spades, Rank::Ace),
        ];
        let hole2 = vec![
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
        ];
        let community = vec![
            Card::new(Suit::Hearts, Rank::Ten),
            Card::new(Suit::Clubs, Rank::Eight),
            Card::new(Suit::Diamonds, Rank::Five),
            Card::new(Suit::Clubs, Rank::Three),
            Card::new(Suit::Diamonds, Rank::Two),
        ];
        let score1 = evaluate_hand(&hole1, &community);
        let score2 = evaluate_hand(&hole2, &community);

        assert_eq!(score1.rank, HandRank::OnePair);
        assert_eq!(score2.rank, HandRank::OnePair);
        assert!(score1 > score2); // Pair of Kings with Ace kicker beats pair with Queen kicker
    }

    #[test]
    fn test_exact_tie() {
        // Same exact hand - should be equal
        let hole = vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::King),
        ];
        let community = vec![
            Card::new(Suit::Diamonds, Rank::Queen),
            Card::new(Suit::Clubs, Rank::Jack),
            Card::new(Suit::Hearts, Rank::Ten),
            Card::new(Suit::Clubs, Rank::Two),
            Card::new(Suit::Diamonds, Rank::Three),
        ];
        let score1 = evaluate_hand(&hole, &community);
        let score2 = evaluate_hand(&hole, &community);

        assert_eq!(score1, score2);
        assert_eq!(score1.rank, HandRank::Straight);
    }
}
