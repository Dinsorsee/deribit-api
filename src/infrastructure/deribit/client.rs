use super::models::auth::RawAuthResponse;
use super::models::index_price::RawIndexPriceResponse;
use super::models::jsonrpc::JsonRpcResponse;
use crate::domain::error::DomainError;
use crate::domain::models::market::IndexPrice;
use crate::domain::models::{auth::AuthToken, market::CurrencyPair};
use crate::domain::ports::exchange::ExchangePort;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::{SplitSink, StreamExt};
use reqwest::{self, Url};
use std::collections::HashMap;
use std::sync::{atomic::AtomicU64, Arc};
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, info};

pub struct DeribitClient {
    ws_sink: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<serde_json::Value>>>>,
    next_id: Arc<AtomicU64>,
    token: Arc<RwLock<Option<AuthToken>>>,
    client_id: String,
    client_secret: String,
}

impl DeribitClient {
    pub async fn connect(
        url: &str,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, DomainError> {
        info!("Connecting to Deribit WebSocket at {}", url);

        let (ws, _) = connect_async(url)
            .await
            .map_err(|e| DomainError::ConnectionError {
                message: e.to_string(),
            })?;

        let (sink, stream) = ws.split();
        let ws_sink: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>> =
            Arc::new(Mutex::new(sink));
        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<serde_json::Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let next_id: Arc<AtomicU64> = Arc::new(AtomicU64::new(1));
        let token: Arc<RwLock<Option<AuthToken>>> = Arc::new(RwLock::new(None));

        // TODO: spawn the background reader task (using the stream<SplitStream> above)
        // pass Arc::clone(&pending) and Arc::clone(&ws_sink) into it

        Ok(Self {
            ws_sink,
            pending,
            next_id,
            token,
            client_id,
            client_secret,
        })
    }

    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        // let http = reqwest::Client::builder()
        //     .build()
        //     .expect("failed to build http client");
        // Self {
        //     http,
        //     base_url,
        //     client_id,
        //     client_secret,
        //     token: tokio::sync::RwLock::new(None),
        // }
        todo!()
    }

    pub async fn authenticate(&self) -> Result<AuthToken, DomainError> {
        info!("Authenticating to Deribit...");

        // let full_url = Url::parse_with_params(
        //     &format!("{}/public/auth", self.base_url),
        //     &[
        //         ("grant_type", "client_credentials"),
        //         ("client_id", &self.client_id),
        //         ("client_secret", &self.client_secret),
        //     ],
        // )
        // .map_err(|e| DomainError::ConfigError {
        //     message: e.to_string(),
        // })?;

        // let response =
        //     self.http
        //         .get(full_url)
        //         .send()
        //         .await
        //         .map_err(|e| DomainError::AuthFailed {
        //             message: e.to_string(),
        //         })?;

        // let raw = response
        //     .json::<JsonRpcResponse<RawAuthResponse>>()
        //     .await
        //     .map_err(|e| DomainError::AuthFailed {
        //         message: e.to_string(),
        //     })?;

        // let auth = raw.into_result().map_err(|e| DomainError::ApiError {
        //     code: e.code,
        //     message: e.message,
        // })?;

        // let token = AuthToken::new(auth.access_token, auth.refresh_token, auth.expires_in)?;

        // *self.token.write().await = Some(token.clone());

        // debug!(
        //     "Successfully authenticated. Token expires in {} seconds",
        //     token.expires_in()
        // );
        // Ok(token)
        todo!()
    }

    pub async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError> {
        info!("Fetching index price for {}", pair.as_api_str());

        // let full_url = Url::parse_with_params(
        //     &format!("{}/public/get_index_price", self.base_url),
        //     &[("index_name", pair.as_api_str())],
        // )
        // .map_err(|e| DomainError::ConfigError {
        //     message: e.to_string(),
        // })?;

        // let response =
        //     self.http
        //         .get(full_url)
        //         .send()
        //         .await
        //         .map_err(|e| DomainError::ExchangeUnavailable {
        //             message: e.to_string(),
        //         })?;

        // let raw = response
        //     .json::<JsonRpcResponse<RawIndexPriceResponse>>()
        //     .await
        //     .map_err(|e| DomainError::ExchangeUnavailable {
        //         message: format!("Failed to parse response: {}", e),
        //     })?;

        // let raw_index_price = raw.into_result().map_err(|e| DomainError::ApiError {
        //     code: e.code,
        //     message: e.message,
        // })?;

        // let index_price = IndexPrice::new(
        //     pair,
        //     raw_index_price.index_price,
        //     raw_index_price.estimated_delivery_price,
        // );

        // debug!(
        //     "Received index price for {}: {}",
        //     pair.as_api_str(),
        //     index_price.price()
        // );
        // Ok(index_price)
        todo!()
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
