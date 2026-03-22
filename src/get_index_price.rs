use crate::models::index_price::{CurrencyPair, RawIndexPriceResponse};
use crate::models::jsonrpc::JSONRPCResponse;
use anyhow::Result;
use reqwest::{self, Url};

pub async fn get_index_price(url: &str) -> Result<()> {
    let client = reqwest::Client::builder().build()?;
    let pair = CurrencyPair::BtcUsdt;
    let full_url = Url::parse_with_params(
        &(url.to_owned() + "/public/get_index_price"),
        &[("index_name", pair.as_api_str())],
    )?;

    let response = client
        .get(full_url.clone())
        .send()
        .await?
        .json::<JSONRPCResponse<RawIndexPriceResponse>>()
        .await?;
    println!("{:?}", response);

    let response_body = response
        .result
        .left_result()
        .map_err(|e| anyhow::anyhow!("API error: {:?}", e))?;

    println!("{}", full_url);
    println!("EDP: {}", response_body.edp);

    if let Some(price) = response_body.price_for(pair.as_api_str()) {
        println!("Index Price ({}): {}", pair.as_api_str(), price)
    }
    Ok(())
}
