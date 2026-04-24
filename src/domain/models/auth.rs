use crate::domain::error::DomainError;

pub struct AuthToken {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

impl AuthToken {
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: i64,
    ) -> Result<Self, DomainError> {
        if access_token.is_empty() {
            return Err(DomainError::TokenMissing);
        }
        Ok(Self {
            access_token,
            refresh_token,
            expires_in,
        })
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }
}
