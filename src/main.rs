use reqwest::{self};
use anyhow::Result;
use dotenv::dotenv;
use std::env;

async fn get_request() -> Result<()>{
    dotenv().ok();

    let response = reqwest::get(env::var("URL").expect("URL Not found")).await?;
    println!("{}", response.status());

    let response_body = response.text().await?;
    println!("\n{}", response_body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    get_request().await?;
    Ok(())
}
