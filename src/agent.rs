use serde::{Serialize, Deserialize};

pub trait Agent<Obs, Act> {
    fn id(&self) -> String;
    fn decide(&mut self, obs: Obs) -> Act;
}

#[derive(Serialize, Deserialize)]
pub struct AgentCommitment {
    pub agent_id: String,
    pub stake: u64,
    pub system_id: String,
}
