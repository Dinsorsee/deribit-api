use super::models::auth::RawAuthResponse;
use super::models::index_price::RawIndexPriceResponse;
use super::models::jsonrpc::JsonRpcResponse;
use crate::domain::error::DomainError;
use crate::domain::models::market::IndexPrice;
use crate::domain::models::{auth::AuthToken, market::CurrencyPair};
use crate::domain::ports::exchange::ExchangePort;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::{self, Url};
use tokio;
use tracing::{debug, info};

pub struct DeribitClient {
    http: reqwest::Client,
    base_url: String,
    client_id: String,
    client_secret: String,
    token: tokio::sync::RwLock<Option<AuthToken>>,
}

impl DeribitClient {
    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        let http = reqwest::Client::builder()
            .build()
            .expect("failed to build http client");
        Self {
            http,
            base_url,
            client_id,
            client_secret,
            token: tokio::sync::RwLock::new(None),
        }
    }

    pub async fn authenticate(&self) -> Result<AuthToken, DomainError> {
        info!("Authenticating to Deribit...");

        let full_url = Url::parse_with_params(
            &format!("{}/public/auth", self.base_url),
            &[
                ("grant_type", "client_credentials"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
            ],
        )
        .map_err(|e| DomainError::ConfigError {
            message: e.to_string(),
        })?;

        let response =
            self.http
                .get(full_url)
                .send()
                .await
                .map_err(|e| DomainError::AuthFailed {
                    message: e.to_string(),
                })?;

        let raw = response
            .json::<JsonRpcResponse<RawAuthResponse>>()
            .await
            .map_err(|e| DomainError::AuthFailed {
                message: e.to_string(),
            })?;

        let auth = raw.into_result().map_err(|e| DomainError::ApiError {
            code: e.code,
            message: e.message,
        })?;

        let token = AuthToken::new(auth.access_token, auth.refresh_token, auth.expires_in)?;

        *self.token.write().await = Some(token.clone());

        debug!(
            "Successfully authenticated. Token expires in {} seconds",
            token.expires_in()
        );
        Ok(token)
    }

    pub async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError> {
        info!("Fetching index price for {}", pair.as_api_str());

        let full_url = Url::parse_with_params(
            &format!("{}/public/get_index_price", self.base_url),
            &[("index_name", pair.as_api_str())],
        )
        .map_err(|e| DomainError::ConfigError {
            message: e.to_string(),
        })?;

        let response =
            self.http
                .get(full_url)
                .send()
                .await
                .map_err(|e| DomainError::ExchangeUnavailable {
                    message: e.to_string(),
                })?;

        let raw = response
            .json::<JsonRpcResponse<RawIndexPriceResponse>>()
            .await
            .map_err(|e| DomainError::ExchangeUnavailable {
                message: format!("Failed to parse response: {}", e),
            })?;

        let raw_index_price = raw.into_result().map_err(|e| DomainError::ApiError {
            code: e.code,
            message: e.message,
        })?;

        let index_price = IndexPrice::new(
            pair,
            raw_index_price.index_price,
            raw_index_price.estimated_delivery_price,
        );

        debug!(
            "Received index price for {}: {}",
            pair.as_api_str(),
            index_price.price()
        );
        Ok(index_price)
    }
}

#[async_trait]
impl ExchangePort for DeribitClient {
    async fn authenticate(&self) -> Result<AuthToken, DomainError> {
        self.authenticate().await
    }

    async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError> {
        self.get_index_price(pair).await
    }
}
