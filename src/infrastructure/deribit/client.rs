use super::models::auth::RawAuthResponse;
use super::models::index_price::RawIndexPriceResponse;
use super::models::jsonrpc::JsonRpcResponse;
use crate::domain::error::DomainError;
use crate::domain::models::market::IndexPrice;
use crate::domain::models::{auth::AuthToken, market::CurrencyPair};
use crate::domain::ports::exchange::ExchangePort;
use crate::infrastructure::deribit::models::jsonrpc::JsonRpcRequest;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::{SplitSink, StreamExt};
use futures_util::SinkExt;
use std::collections::HashMap;
use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc};
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex, RwLock};
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::{debug, info, warn};

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

        Self::spawn_reader(Arc::clone(&pending), stream);

        info!("WebSocket connection established");

        Ok(Self {
            ws_sink,
            pending,
            next_id,
            token,
            client_id,
            client_secret,
        })
    }

    fn spawn_reader(
        pending: Arc<Mutex<HashMap<u64, oneshot::Sender<serde_json::Value>>>>,
        mut stream: impl StreamExt<Item = Result<Message, tungstenite::Error>> + Send + 'static + Unpin,
    ) {
        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(value) => {
                                if let Some(id) = value["id"].as_u64() {
                                    let mut map = pending.lock().await;
                                    if let Some(sender) = map.remove(&id) {
                                        let _ = sender.send(value);
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Failed to parse incoming WebSocket message: {}", e)
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        warn!("WebSocket connection closed by server");
                        break;
                    }
                    Err(e) => {
                        warn!("WebSocket read error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
    }

    async fn send_request<P, R>(&self, method: &str, params: P) -> Result<R, DomainError>
    where
        P: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = JsonRpcRequest::new(id, method, params);

        // A serialisation failure is a malformed-request problem at the exchange
        // boundary — ExchangeUnavailable, NOT ConfigError (that variant is for
        // missing/invalid environment config;
        let json = serde_json::to_string(&request).map_err(|e| DomainError::ConfigError {
            message: format!("failed to serialise request: {}", e),
        })?;

        let (tx, rx) = oneshot::channel::<serde_json::Value>();
        {
            let mut map = self.pending.lock().await;
            map.insert(id, tx);
        };

        let send_result = {
            let mut sink = self.ws_sink.lock().await;
            sink.send(Message::Text(json)).await
        };

        if let Err(e) = send_result {
            self.pending.lock().await.remove(&id);
            return Err(DomainError::ExchangeUnavailable {
                message: e.to_string(),
            });
        }

        let value = rx.await.map_err(|_| DomainError::ExchangeUnavailable {
            message: "response channel closed before reply arrived".to_string(),
        })?;

        let response: JsonRpcResponse<R> =
            serde_json::from_value(value).map_err(|e| DomainError::ExchangeUnavailable {
                message: format!("failed to parse response: {}", e),
            })?;

        response.into_result().map_err(|e| DomainError::ApiError {
            code: e.code,
            message: e.message,
        })
    }

    pub async fn authenticate(&self) -> Result<AuthToken, DomainError> {
        info!("Authenticating to Deribit...");

        #[derive(serde::Serialize)]
        struct AuthParams<'a> {
            grant_type: &'a str,
            client_id: &'a str,
            client_secret: &'a str,
        }

        let raw = self
            .send_request::<AuthParams, RawAuthResponse>(
                "public/auth",
                AuthParams {
                    grant_type: "client_credentials",
                    client_id: &self.client_id,
                    client_secret: &self.client_secret,
                },
            )
            .await?;

        let token = AuthToken::new(raw.access_token, raw.refresh_token, raw.expires_in)?;

        *self.token.write().await = Some(token.clone());

        debug!(
            "Successfully authenticated. Token expires in {} hours",
            token.expires_in() / (60 * 60)
        );
        Ok(token)
    }

    pub async fn get_index_price(&self, pair: CurrencyPair) -> Result<IndexPrice, DomainError> {
        info!("Fetching index price for {}", pair.as_api_str());

        #[derive(serde::Serialize)]
        struct IndexPriceParams<'a> {
            index_name: &'a str,
        }

        let raw = self
            .send_request::<IndexPriceParams, RawIndexPriceResponse>(
                "public/get_index_price",
                IndexPriceParams {
                    index_name: pair.as_api_str(),
                },
            )
            .await?;

        let price = IndexPrice::new(pair, raw.index_price, raw.estimated_delivery_price);
        debug!(
            "Received index price for {}: {}",
            pair.as_api_str(),
            price.price()
        );
        Ok(price)
    }

    async fn ensure_authenticated(&self) -> Result<(), DomainError> {
        if self.token.read().await.is_none() {
            return Err(DomainError::NotAuthenticated);
        }
        Ok(())
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
