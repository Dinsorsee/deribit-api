use anyhow::Result;
use dotenv::dotenv;
use reqwest::{self};
use reqwest::{StatusCode, Url};
use std::env;

pub async fn get_access_token() -> Result<()> {
    dotenv().ok();

    let client = reqwest::Client::builder().build()?;
    let client_id = env::var("CLIENT_ID")?;
    let client_secret = env::var("CLIENT_SECRET")?;
    let url = Url::parse_with_params(
        &(env::var("URL").expect("URL Not found") + "/public/auth"),
        &[
            ("grant_type", "client_credentials"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ],
    )?;

    let response = client.get(url).send().await?;
    match response.status() {
        StatusCode::OK => {
            println!(
                "Status code: {} - ✅ Connection Completed",
                response.status()
            )
        }
        StatusCode::NOT_FOUND => {
            println!("Status code: {} - ❌ Connection failed", response.status())
        }
        _ => {
            println!("Status code: {} ❌ ", response.status())
        }
    }
    let response_body = response.text().await?;
    println!("\n{:#?}", response_body);
    Ok(())
}
