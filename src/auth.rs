use super::models::auth::AuthResponse;
use crate::models::jsonrpc::JSONRPCResponse;
use anyhow::Result;
use dotenv::dotenv;
use reqwest::{self, Url};
use std::env;

pub async fn get_token(url: &str) -> Result<()> {
    dotenv().ok();

    let client = reqwest::Client::builder().build()?;
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
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
        .json::<JSONRPCResponse<AuthResponse>>()
        .await?;

    let response_body = match response.result.clone().left() {
        Some(_) => response.result.left().unwrap(),
        None => panic!("Panic!"),
    };

    println!("\naccess_token: {:#?}", response_body.access_token()?);
    println!("\nrefresh_token: {:#?}", response_body.refresh_token()?);
    Ok(())
}
