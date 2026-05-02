use super::model::{AppConfig, DeribitConfig};
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

pub fn load() -> Result<AppConfig> {
    dotenv().ok();

    let deribit = DeribitConfig {
        url: env::var("URL").expect("missing required env variable: URL"),
        client_id: env::var("CLIENT_ID").expect("missing required env variable: CLIENT_ID"),
        client_secret: env::var("CLIENT_SECRET")
            .expect("missing required env variable: CLIENT_SECRET"),
    };

    Ok(AppConfig { deribit })
}
