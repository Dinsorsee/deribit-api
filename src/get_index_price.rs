use crate::models::index_price::{CurrencyPair, RawIndexPriceResponse};
use crate::models::jsonrpc::JsonRpcResponse;
use anyhow::Result;
use reqwest::{self, Url};

pub async fn get_index_price(url: &str) -> Result<()> {
    let client = reqwest::Client::builder().build()?;
    let pair = CurrencyPair::BtcUsdt;
    let full_url = Url::parse_with_params(
        &(url.to_owned() + "/public/get_index_price"),
        &[("index_name", pair.as_api_str())],
    )?;

    let response = client.get(full_url).send().await?;
    let raw = response
        .json::<JsonRpcResponse<RawIndexPriceResponse>>()
        .await?;

    let index = raw.into_result().map_err(|e| anyhow::anyhow!("{}", e))?;

    println!(
        "\n{}: {}",
        pair.as_api_str(),
        index.estimated_delivery_price
    );
    Ok(())
}
