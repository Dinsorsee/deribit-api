use crate::domain::error::DomainError;
use crate::domain::models::auth::AuthToken;
use crate::domain::models::market::{CurrencyPair, IndexPrice};
use async_trait::async_trait;

#[async_trait]
pub trait ExchangePort: Send + Sync {
    async fn authenticate(&self) -> Result<AuthToken, DomainError>;
    async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError>;
}
