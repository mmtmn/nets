use std::collections::HashMap;

pub struct Ledger {
    balances: HashMap<String, u64>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { balances: HashMap::new() }
    }

    pub fn credit(&mut self, agent: &str, amount: u64) {
        *self.balances.entry(agent.to_string()).or_insert(0) += amount;
    }

    pub fn slash(&mut self, agent: &str, amount: u64) {
        let entry = self.balances.entry(agent.to_string()).or_insert(0);
        *entry = entry.saturating_sub(amount);
    }

    pub fn balance(&self, agent: &str) -> u64 {
        *self.balances.get(agent).unwrap_or(&0)
    }

    pub fn snapshot(&self) -> Vec<(String, u64)> {
        self.balances.iter().map(|(k,v)| (k.clone(), *v)).collect()
    }

    pub fn restore(&mut self, data: Vec<(String, u64)>) {
        self.balances = data.into_iter().collect();
    }
}
