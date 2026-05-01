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
use std::env;
use tokio;

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
        Ok(token)
    }

    pub async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError> {
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

pub async fn get_token(url: &str) -> Result<()> {
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;

    let client = reqwest::Client::builder().build()?;

    let full_url = Url::parse_with_params(
        &(url.to_owned() + "/public/auth"),
        &[
            ("grant_type", "client_credentials"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ],
    )?;

    let response = client.get(full_url).send().await?;
    let raw = response.json::<JsonRpcResponse<RawAuthResponse>>().await?;

    let auth = raw.into_result().map_err(|e| anyhow::anyhow!("{}", e))?;

    println!("\naccess_token: {:#?}", auth.access_token);
    println!("\nrefresh_token: {:#?}", auth.refresh_token);
    Ok(())
}

pub async fn get_index_price(url: &str) -> Result<()> {
    let client = reqwest::Client::builder().build()?;
    let pair = CurrencyPair::BtcUsdt;
    let full_url = Url::parse_with_params(
        &(url.to_owned() + "/public/get_index_price"),
        &[("index_name", pair.as_api_str())],
    )?;

    let response = client.get(full_url).send().await?;
    let raw = response
        .json::<JsonRpcResponse<RawIndexPriceResponse>>()
        .await?;

    let index = raw.into_result().map_err(|e| anyhow::anyhow!("{}", e))?;

    println!(
        "\n{}: {}",
        pair.as_api_str(),
        index.estimated_delivery_price
    );
    Ok(())
}
