#[derive(Debug, Clone)]
pub struct AppConfig {
    pub deribit: DeribitConfig,
}

#[derive(Debug, Clone)]
pub struct DeribitConfig {
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
}
