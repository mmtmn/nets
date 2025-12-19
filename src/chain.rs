use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchCommitment {
    pub system_id: String,
    pub league_id: String,
    pub merkle_root: [u8; 32],
    pub agent_ids: Vec<String>,
    pub scores: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettlementTx {
    pub commitment_hash: [u8; 32],
    pub payouts: Vec<(String, u64)>,
}
