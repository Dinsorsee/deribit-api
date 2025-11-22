use anyhow::Result;
use deribit_api::auth::get_token;
use deribit_api::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1)
        }
    };

    info!("Loaded ENV config: success");

    get_token(&dotenvy_env.deribit_api.url).await?;
    Ok(())
}
