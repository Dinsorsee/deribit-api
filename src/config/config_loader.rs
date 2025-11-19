use super::config_model::{DeribitApi, DotEnvyConfig};
use anyhow::Result;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let deribit_api = DeribitApi {
        url: std::env::var("URL").expect("URL is invalid"),
    };

    Ok(DotEnvyConfig { deribit_api })
}
