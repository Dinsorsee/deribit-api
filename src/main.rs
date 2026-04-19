use anyhow::Result;
use deribit_api::config::loader;
use deribit_api::infrastructure::deribit::client::{get_index_price, get_token};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1)
        }
    };

    info!("Loaded ENV config: success");

    get_token(&dotenvy_env.deribit_api.url).await?;
    get_index_price(&dotenvy_env.deribit_api.url).await?;
    Ok(())
}
