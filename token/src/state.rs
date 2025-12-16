//! Token contract state using Linera views

use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    views::{linera_views, RegisterView, RootView, ViewStorageContext},
};

/// Token state - chip balance for a player
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct TokenState {
    /// Total balance
    pub balance: RegisterView<Amount>,
    /// Amount locked in games
    pub locked: RegisterView<Amount>,
    /// Owner of these tokens
    pub owner: RegisterView<Option<AccountOwner>>,
}
