use crate::domain::error::DomainError;
use crate::domain::models::account::AccountSummary;
use crate::domain::models::auth::AuthToken;
use crate::domain::models::market::{CurrencyPair, IndexPrice};
use async_trait::async_trait;

// async-trait adds #[must_use] to methods returning futures, which are already must-use.
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait ExchangePort: Send + Sync {
    async fn authenticate(&self) -> Result<AuthToken, DomainError>;
    async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError>;
    async fn get_account_summary(&self, currency: &str) -> Result<AccountSummary, DomainError>;
}
