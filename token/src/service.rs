#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use self::state::TokenState;
use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_poker_token::TokenAbi;
use linera_sdk::{linera_base_types::WithServiceAbi, views::View, Service, ServiceRuntime};

pub struct TokenService {
    state: Arc<TokenState>,
}

linera_sdk::service!(TokenService);

impl WithServiceAbi for TokenService {
    type Abi = TokenAbi;
}

impl Service for TokenService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self {
            state: Arc::new(state),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
            },
            MutationRoot,
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}

struct QueryRoot {
    state: Arc<TokenState>,
}

#[Object]
impl QueryRoot {
    /// Get full token state
    async fn state(&self) -> TokenStateView {
        let balance = *self.state.balance.get();
        let locked = *self.state.locked.get();
        TokenStateView {
            balance: balance.to_string(),
            locked: locked.to_string(),
            available: balance.saturating_sub(locked).to_string(),
        }
    }

    /// Get total balance
    async fn balance(&self) -> String {
        self.state.balance.get().to_string()
    }

    /// Get locked amount
    async fn locked(&self) -> String {
        self.state.locked.get().to_string()
    }

    /// Get available (balance - locked)
    async fn available(&self) -> String {
        let balance = *self.state.balance.get();
        let locked = *self.state.locked.get();
        balance.saturating_sub(locked).to_string()
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Placeholder mutation
    async fn noop(&self) -> bool {
        true
    }
}

#[derive(async_graphql::SimpleObject)]
struct TokenStateView {
    balance: String,
    locked: String,
    available: String,
}
