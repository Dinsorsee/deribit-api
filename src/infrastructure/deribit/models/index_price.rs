use serde::{Deserialize, Serialize};

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
    pub estimated_delivery_price: f64,
    pub index_price: f64,
}
