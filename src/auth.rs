use super::models::auth::RawAuthResponse;
use crate::models::jsonrpc::JsonRpcResponse;
use anyhow::Result;
use reqwest::{self, Url};
use std::env;

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
