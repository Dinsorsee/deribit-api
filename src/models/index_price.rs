use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CurrencyPair {
    #[serde(alias = "btc_usdt")]
    BtcUsdt,
}

impl CurrencyPair {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            CurrencyPair::BtcUsdt => "btc_usdt",
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RawIndexPriceResponse {
    pub edp: f64,
    #[serde(flatten)]
    pub index_prices: HashMap<String, serde_json::Value>,
}

impl RawIndexPriceResponse {
    pub fn price_for(&self, index_name: &str) -> Option<f64> {
        self.index_prices.get(index_name).and_then(|v| v.as_f64())
    }
}
