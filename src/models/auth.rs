use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RawAuthResponse {
    pub access_token: Option<String>,
    pub refresh_token: String,
    pub expires_in: i64,
    pub scope: String,
    pub token_type: String,
}
