#[derive(Clone, Copy)]
pub enum CurrencyPair {
    BtcUsdt,
}

impl CurrencyPair {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            CurrencyPair::BtcUsdt => "btc_usdt",
        }
    }

    pub fn instrument_name(&self) -> &'static str {
        match self {
            CurrencyPair::BtcUsdt => "BTC-PERPETUAL",
        }
    }
}

pub struct IndexPrice {
    pair: CurrencyPair,
    price: f64,
    estimated_delivery_price: f64,
}

impl IndexPrice {
    pub fn new(pair: CurrencyPair, price: f64, estimated_delivery_price: f64) -> Self {
        Self {
            pair,
            price,
            estimated_delivery_price,
        }
    }

    pub fn pair(&self) -> &CurrencyPair {
        &self.pair
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn estimated_delivery_price(&self) -> f64 {
        self.estimated_delivery_price
    }
}
