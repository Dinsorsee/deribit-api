use super::models::auth::RawAuthResponse;
use crate::models::jsonrpc::JSONRPCResponse;
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

    let response = client
        .get(full_url.clone())
        .send()
        .await?
        .json::<JSONRPCResponse<RawAuthResponse>>()
        .await?;

    let response_body = response
        .result
        .left_result()
        .map_err(|e| anyhow::anyhow!("API error: {:?}", e))?;

    let access_token = response_body
        .access_token
        .clone()
        .ok_or_else(|| anyhow::anyhow!("missing access_token"))?;

    println!("\naccess_token: {:#?}", access_token);
    println!("\nrefresh_token: {:#?}", response_body.refresh_token);

    Ok(())
}
