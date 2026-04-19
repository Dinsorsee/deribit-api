use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: JsonRpcVersion,
    pub testnet: bool,
    pub result: Option<T>,
    pub error: Option<ApiError>,
    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
}

impl<T> JsonRpcResponse<T> {
    pub fn into_result(self) -> Result<T, ApiError> {
        match (self.result, self.error) {
            (Some(result), _) => Ok(result),
            (None, Some(error)) => Err(error),
            (None, None) => Err(ApiError {
                code: -1,
                message: "API response contained neither 'result' nor 'error'".to_string(),
            }),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApiError {
    pub code: i64,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Deribit Api Error {}: {}", self.code, self.message)
    }
}
