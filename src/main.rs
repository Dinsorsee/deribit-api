use anyhow::Result;
use deribit_api::config::loader;
use deribit_api::domain::models::market::CurrencyPair;
use deribit_api::infrastructure::deribit::client::DeribitClient;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_config = match loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1)
        }
    };
    info!("Loaded ENV config: success");

    let url = app_config.deribit.url;
    let client_id = app_config.deribit.client_id;
    let client_secret = app_config.deribit.client_secret;

    let client = DeribitClient::new(url, client_id, client_secret);
    let pair = CurrencyPair::BtcUsdt;

    client.authenticate().await?;
    client.get_index_price(pair).await?;
    Ok(())
}
