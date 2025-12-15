#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use self::state::TableState;
use async_graphql::{EmptySubscription, Enum, InputObject, Object, Schema, Request, Response};
use linera_poker_table::{TableAbi, TableOperation, BetAction, Card, CardReveal};
use linera_poker_shared::{Suit, Rank};
use linera_sdk::{
    linera_base_types::{Amount, ApplicationId, ChainId, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};

pub struct TableService {
    state: Arc<TableState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(TableService);

impl WithServiceAbi for TableService {
    type Abi = TableAbi;
}

impl Service for TableService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TableState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            QueryRoot { state: self.state.clone() },
            MutationRoot { runtime: self.runtime.clone() },
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct QueryRoot {
    state: Arc<TableState>,
}

#[Object]
impl QueryRoot {
    /// Get full table state
    async fn state(&self) -> TableStateView {
        TableStateView {
            game_id: *self.state.game_id.get(),
            phase: format!("{:?}", self.state.phase.get()),
            players: self.state.players.get().iter().map(|p| PlayerInfoView {
                seat: format!("{:?}", p.seat),
                chain_id: p.chain_id.to_string(),
                stake: p.stake.to_string(),
                has_folded: p.has_folded,
                current_bet: p.current_bet.to_string(),
                has_revealed: p.has_revealed,
            }).collect(),
            pot: self.state.pot.get().to_string(),
            current_bet: self.state.current_bet.get().to_string(),
            min_raise: self.state.min_raise.get().to_string(),
            community_cards: self.state.community_cards.get().iter().map(|c| CardView {
                suit: format!("{:?}", c.suit),
                rank: format!("{:?}", c.rank),
            }).collect(),
            turn_seat: self.state.turn_seat.get().map(|s| format!("{:?}", s)),
            winner: self.state.winner.get().map(|s| format!("{:?}", s)),
            min_stake: self.state.min_stake.get().to_string(),
            max_stake: self.state.max_stake.get().to_string(),
            small_blind: self.state.small_blind.get().to_string(),
            big_blind: self.state.big_blind.get().to_string(),
            dealer_button: self.state.dealer_button.get().map(|s| format!("{:?}", s)),
            deck_seed: self.state.deck_seed.get().clone(),
            dealer_secret: self.state.dealer_secret.get().clone(),
        }
    }

    /// Get current game ID
    async fn game_id(&self) -> u64 {
        *self.state.game_id.get()
    }

    /// Get current phase
    async fn phase(&self) -> String {
        format!("{:?}", self.state.phase.get())
    }

    /// Get pot amount
    async fn pot(&self) -> String {
        self.state.pot.get().to_string()
    }

    /// Get players
    async fn players(&self) -> Vec<PlayerInfoView> {
        self.state.players.get().iter().map(|p| PlayerInfoView {
            seat: format!("{:?}", p.seat),
            chain_id: p.chain_id.to_string(),
            stake: p.stake.to_string(),
            has_folded: p.has_folded,
            current_bet: p.current_bet.to_string(),
            has_revealed: p.has_revealed,
        }).collect()
    }

    /// Get whose turn it is
    async fn turn_seat(&self) -> Option<String> {
        self.state.turn_seat.get().map(|s| format!("{:?}", s))
    }

    /// Get winner
    async fn winner(&self) -> Option<String> {
        self.state.winner.get().map(|s| format!("{:?}", s))
    }

    /// Get community cards
    async fn community_cards(&self) -> Vec<CardView> {
        self.state.community_cards.get().iter().map(|c| CardView {
            suit: format!("{:?}", c.suit),
            rank: format!("{:?}", c.rank),
        }).collect()
    }
}

struct MutationRoot {
    runtime: Arc<ServiceRuntime<TableService>>,
}

#[Object]
impl MutationRoot {
    /// Join table with stake amount
    async fn join_table(&self, player_chain_id: String, stake: String, hand_app_id: Option<String>) -> bool {
        let player_chain = match player_chain_id.parse::<ChainId>() {
            Ok(c) => c,
            Err(_) => return false,
        };
        let stake_amount: u64 = stake.parse().unwrap_or(0);
        let app_id = hand_app_id
            .and_then(|s| s.parse::<ApplicationId>().ok())
            .unwrap_or_else(|| self.runtime.application_id().forget_abi());

        let operation = TableOperation::RelayJoinTable {
            player_chain,
            stake: Amount::from_tokens(stake_amount.into()),
            hand_app_id: app_id,
        };
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Place a betting action
    async fn bet(&self, player_chain_id: String, action: BetActionInput) -> bool {
        let player_chain = match player_chain_id.parse::<ChainId>() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let bet_action = match action.action_type {
            BetActionType::Check => BetAction::Check,
            BetActionType::Call => BetAction::Call,
            BetActionType::Raise => {
                let amount = action.amount.unwrap_or_default().parse::<u128>().unwrap_or(0);
                BetAction::Raise(Amount::from_attos(amount))
            }
            BetActionType::AllIn => BetAction::AllIn,
            BetActionType::Fold => BetAction::Fold,
        };

        let operation = TableOperation::RelayBetAction {
            player_chain,
            game_id: 0, // Will be validated by contract
            action: bet_action,
        };
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Reveal cards for showdown
    async fn reveal_cards(&self, player_chain_id: String, cards: Vec<CardInput>) -> bool {
        let player_chain = match player_chain_id.parse::<ChainId>() {
            Ok(c) => c,
            Err(_) => return false,
        };

        let revealed_cards: Vec<Card> = cards.iter().map(|c| Card {
            suit: match c.suit.as_str() {
                "Hearts" => Suit::Hearts,
                "Diamonds" => Suit::Diamonds,
                "Clubs" => Suit::Clubs,
                _ => Suit::Spades,
            },
            rank: parse_rank(&c.rank),
        }).collect();

        // Create empty proofs for now (verification disabled for demo)
        let proofs: Vec<CardReveal> = revealed_cards.iter().map(|card| CardReveal {
            card: *card,
            secret: vec![],
        }).collect();

        let operation = TableOperation::RelayRevealCards {
            player_chain,
            game_id: 0,
            cards: revealed_cards,
            proofs,
        };
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Start a new game
    async fn start_new_game(&self) -> bool {
        let operation = TableOperation::StartNewGame;
        self.runtime.schedule_operation(&operation);
        true
    }
}

/// GraphQL input for bet actions
#[derive(InputObject)]
struct BetActionInput {
    action_type: BetActionType,
    amount: Option<String>,
}

/// Bet action types for GraphQL
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum BetActionType {
    Check,
    Call,
    Raise,
    AllIn,
    Fold,
}

/// GraphQL input for cards
#[derive(InputObject)]
struct CardInput {
    suit: String,
    rank: String,
}

fn parse_rank(rank_str: &str) -> Rank {
    match rank_str {
        "Two" | "2" => Rank::Two,
        "Three" | "3" => Rank::Three,
        "Four" | "4" => Rank::Four,
        "Five" | "5" => Rank::Five,
        "Six" | "6" => Rank::Six,
        "Seven" | "7" => Rank::Seven,
        "Eight" | "8" => Rank::Eight,
        "Nine" | "9" => Rank::Nine,
        "Ten" | "10" => Rank::Ten,
        "Jack" | "J" => Rank::Jack,
        "Queen" | "Q" => Rank::Queen,
        "King" | "K" => Rank::King,
        "Ace" | "A" | "14" => Rank::Ace,
        _ => Rank::Two,
    }
}

#[derive(async_graphql::SimpleObject)]
struct TableStateView {
    game_id: u64,
    phase: String,
    players: Vec<PlayerInfoView>,
    pot: String,
    current_bet: String,
    min_raise: String,
    community_cards: Vec<CardView>,
    turn_seat: Option<String>,
    winner: Option<String>,
    min_stake: String,
    max_stake: String,
    /// Small blind amount
    small_blind: String,
    /// Big blind amount
    big_blind: String,
    /// Current dealer button position
    dealer_button: Option<String>,
    /// Deck seed for provable fairness
    deck_seed: Vec<u8>,
    /// Dealer secret for card commitments
    dealer_secret: Vec<u8>,
}

#[derive(async_graphql::SimpleObject)]
struct PlayerInfoView {
    seat: String,
    chain_id: String,
    stake: String,
    has_folded: bool,
    current_bet: String,
    has_revealed: bool,
}

#[derive(async_graphql::SimpleObject)]
struct CardView {
    suit: String,
    rank: String,
}
