use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawAccountSummaryResponse {
    pub currency: String,
    pub balance: f64,
    pub available_funds: f64,
    pub equity: f64,
}
