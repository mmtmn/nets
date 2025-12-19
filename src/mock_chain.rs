use std::collections::HashMap;
use sha2::{Sha256, Digest};

use crate::chain::{MatchCommitment, SettlementTx};
use crate::chain_adapter::ChainAdapter;

pub struct MockChain {
    pub balances: HashMap<String, u64>,
}

impl MockChain {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
}

impl ChainAdapter for MockChain {
    fn post_commitment(&mut self, commitment: &MatchCommitment) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(&bincode::serialize(commitment).unwrap());
        h.finalize().into()
    }

    fn settle(&mut self, tx: SettlementTx) {
        for (agent, amount) in tx.payouts {
            *self.balances.entry(agent).or_insert(0) += amount;
        }
    }
}
