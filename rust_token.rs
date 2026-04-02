use std::collections::HashMap;

pub struct RustToken {
    pub name: &'static str,
    pub symbol: &'static str,
    pub total_supply: f64,
    balances: HashMap<String, f64>,
}

impl RustToken {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("GENESIS".to_string(), 21_000_000.0);
        RustToken {
            name: "RustChainToken",
            symbol: "RCT",
            total_supply: 21_000_000.0,
            balances,
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: f64) -> bool {
        let bal = self.balances.get(from).copied().unwrap_or(0.0);
        if bal < amount { return false; }
        *self.balances.entry(from.to_string()).or_insert(0.0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0.0) += amount;
        true
    }
}
