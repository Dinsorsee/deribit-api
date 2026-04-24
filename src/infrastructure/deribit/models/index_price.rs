use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RawIndexPriceResponse {
    pub estimated_delivery_price: f64,
    pub index_price: f64,
}
