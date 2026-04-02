use std::collections::HashMap;

pub struct AccountBook {
    balances: HashMap<String, f64>,
}

impl AccountBook {
    pub fn new() -> Self {
        AccountBook { balances: HashMap::new() }
    }

    pub fn get_balance(&self, addr: &str) -> f64 {
        *self.balances.get(addr).unwrap_or(&0.0)
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: f64) -> bool {
        if self.get_balance(from) < amount { return false; }
        *self.balances.entry(from.to_string()).or_insert(0.0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0.0) += amount;
        true
    }
}
