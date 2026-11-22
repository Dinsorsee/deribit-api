use chrono::{DateTime, Utc};

use crate::domain::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderType {
    Market,
    Limit { price: f64 },
}

#[derive(Debug, Clone)]
pub struct Order {
    instrument_name: String,
    amount: f64,
    side: OrderSide,
    order_type: OrderType,
}

impl Order {
    pub fn new_market_buy(
        instrument_name: impl Into<String>,
        amount: f64,
    ) -> Result<Self, DomainError> {
        if !amount.is_finite() || amount <= 0.0 {
            return Err(DomainError::InvalidOrderAmount);
        }

        Ok(Self {
            instrument_name: instrument_name.into(),
            amount,
            side: OrderSide::Buy,
            order_type: OrderType::Market,
        })
    }

    pub fn instrument_name(&self) -> &str {
        &self.instrument_name
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn side(&self) -> OrderSide {
        self.side
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type
    }
}

#[derive(Debug, Clone)]
pub enum OrderState {
    Open,
    Filled,
    Rejected,
    Cancelled,
    Untriggered,
}

#[derive(Debug, Clone)]
pub struct OrderResult {
    pub order_id: String,
    pub instrument_name: String,
    pub direction: OrderSide,
    pub state: OrderState,
    pub amount: f64,
    pub filled_amount: f64,
    pub average_price: f64,
    pub executed_at: DateTime<Utc>,
}
