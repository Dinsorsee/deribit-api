use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Authentication failed: {message}")]
    AuthFailed { message: String },
    #[error("Api error (code {code}): {message}")]
    ApiError { code: i64, message: String },
    #[error("Token is missing")]
    TokenMissing,
    #[error("Not authenticated: no token provided")]
    NotAuthenticated,
    #[error("Insufficient funds: available balance {available:.2}, required {required:.2}")]
    InsufficientFunds { available: f64, required: f64 },
    #[error("Exchange rejected the order: {reason}")]
    OrderRejected { reason: String },
    #[error("Database operation failed: {message}")]
    DatabaseError { message: String },
    #[error("Missing or invalid environment variable: {message}")]
    ConfigError { message: String },
}
