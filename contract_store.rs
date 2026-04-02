use std::collections::HashMap;

pub struct ContractStore {
    store: HashMap<String, HashMap<String, String>>,
}

impl ContractStore {
    pub fn new() -> Self {
        ContractStore { store: HashMap::new() }
    }

    pub fn set(&mut self, contract: &str, key: &str, value: &str) {
        self.store.entry(contract.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, contract: &str, key: &str) -> Option<String> {
        self.store.get(contract)?.get(key).cloned()
    }
}
