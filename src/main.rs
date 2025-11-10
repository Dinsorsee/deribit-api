use reqwest::{self};
use anyhow::Result;
use dotenv::dotenv;

async fn get_request() -> Result<()>{
    let response = reqwest::get("https://test.deribit.com/api/v2").await?;
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
