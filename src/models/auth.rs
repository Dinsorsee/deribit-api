use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AuthResponse {
    access_token: Option<String>,
    expires_in: i64,
    refresh_token: String,
    scope: String,
    state: Option<String>,
    token_type: String,
}

impl AuthResponse {
    pub fn access_token(&self) -> Result<String> {
        let result: String = match &self.access_token {
            Some(token) => {
                format!("{}", token)
            }
            None => panic!("None"),
        };

        Ok(result)
    }

    pub fn refresh_token(&self) -> Result<String> {
        let result: String = format!("{}", &self.refresh_token);
        Ok(result)
    }
}
