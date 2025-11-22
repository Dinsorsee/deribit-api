use crate::models::{Either, Request};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JSONRPCRequest<Q: Request> {
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "crate::models::Request::no_payload")]
    pub params: Q,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCResponse<R> {
    pub jsonrpc: JSONRPCVersion,
    pub testnet: bool,
    #[serde(alias = "error")]
    pub result: Either<R, ErrorDetail>,
    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JSONRPCSuccessResponse<R> {
    pub jsonrpc: JSONRPCVersion,
    pub id: i64,
    pub testnet: bool,
    pub result: R,
    pub us_in: u64,
    pub us_out: u64,
    pub us_diff: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum JSONRPCVersion {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ErrorDetail {
    pub code: i64,
    pub message: String,
}
