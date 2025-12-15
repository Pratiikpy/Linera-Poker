#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use self::state::HandState;
use async_graphql::{EmptySubscription, Enum, InputObject, Object, Schema, Request, Response};
use linera_poker_hand::{HandAbi, HandOperation, BetAction};
use linera_sdk::{
    linera_base_types::{Amount, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};

pub struct HandService {
    state: Arc<HandState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(HandService);

impl WithServiceAbi for HandService {
    type Abi = HandAbi;
}

impl Service for HandService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = HandState::load(runtime.root_view_storage_context())
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
    state: Arc<HandState>,
}

#[Object]
impl QueryRoot {
    /// Get full hand state (PRIVATE - only visible on this chain!)
    async fn state(&self) -> HandStateView {
        HandStateView {
            game_id: *self.state.game_id.get(),
            table_chain: self.state.table_chain.get().map(|c| c.to_string()),
            seat: self.state.seat.get().map(|s| format!("{:?}", s)),
            hole_cards: self.state.hole_cards.get().iter().map(|c| CardView {
                suit: format!("{:?}", c.suit),
                rank: format!("{:?}", c.rank),
            }).collect(),
            community_cards: self.state.community_cards.get().iter().map(|c| CardView {
                suit: format!("{:?}", c.suit),
                rank: format!("{:?}", c.rank),
            }).collect(),
            current_bet: self.state.current_bet.get().to_string(),
            my_turn: *self.state.my_turn.get(),
            game_result: self.state.game_result.get().as_ref().map(|r| GameResultView {
                won: r.won,
                payout: r.payout.to_string(),
            }),
        }
    }

    /// Get current game ID
    async fn game_id(&self) -> Option<u64> {
        *self.state.game_id.get()
    }

    /// Get our hole cards (PRIVATE!)
    async fn hole_cards(&self) -> Vec<CardView> {
        self.state.hole_cards.get().iter().map(|c| CardView {
            suit: format!("{:?}", c.suit),
            rank: format!("{:?}", c.rank),
        }).collect()
    }

    /// Get community cards
    async fn community_cards(&self) -> Vec<CardView> {
        self.state.community_cards.get().iter().map(|c| CardView {
            suit: format!("{:?}", c.suit),
            rank: format!("{:?}", c.rank),
        }).collect()
    }

    /// Is it our turn?
    async fn my_turn(&self) -> bool {
        *self.state.my_turn.get()
    }

    /// Get game result
    async fn game_result(&self) -> Option<GameResultView> {
        self.state.game_result.get().as_ref().map(|r| GameResultView {
            won: r.won,
            payout: r.payout.to_string(),
        })
    }
}

struct MutationRoot {
    runtime: Arc<ServiceRuntime<HandService>>,
}

#[Object]
impl MutationRoot {
    /// Join a poker table with the specified stake
    async fn join_table(&self, stake: String) -> bool {
        let stake_amount: u64 = stake.parse().unwrap_or(0);
        let operation = HandOperation::JoinTable { stake: stake_amount };
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Place a bet action (CHECK, CALL, RAISE, ALL_IN, FOLD)
    async fn bet(&self, action: BetActionInput) -> bool {
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
        let operation = HandOperation::Bet { action: bet_action };
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Reveal hole cards for showdown
    async fn reveal(&self) -> bool {
        let operation = HandOperation::Reveal;
        self.runtime.schedule_operation(&operation);
        true
    }

    /// Leave the table
    async fn leave_table(&self) -> bool {
        let operation = HandOperation::LeaveTable;
        self.runtime.schedule_operation(&operation);
        true
    }
}

/// GraphQL input for bet actions
#[derive(InputObject)]
struct BetActionInput {
    /// The type of action: CHECK, CALL, RAISE, ALL_IN, FOLD
    action_type: BetActionType,
    /// Amount for RAISE action (as string to handle large numbers)
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

#[derive(async_graphql::SimpleObject)]
struct HandStateView {
    game_id: Option<u64>,
    table_chain: Option<String>,
    seat: Option<String>,
    hole_cards: Vec<CardView>,
    community_cards: Vec<CardView>,
    current_bet: String,
    my_turn: bool,
    game_result: Option<GameResultView>,
}

#[derive(async_graphql::SimpleObject)]
struct CardView {
    suit: String,
    rank: String,
}

#[derive(async_graphql::SimpleObject)]
struct GameResultView {
    won: bool,
    payout: String,
}
