pub struct AccountSummary {
    pub currency: String,
    pub balance: f64,
    pub available_funds: f64,
    pub equity: f64,
}

impl AccountSummary {
    pub fn new(currency: String, balance: f64, available_funds: f64, equity: f64) -> Self {
        Self {
            currency,
            balance,
            available_funds,
            equity,
        }
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn available_funds(&self) -> f64 {
        self.available_funds
    }

    pub fn equity(&self) -> f64 {
        self.equity
    }
}
