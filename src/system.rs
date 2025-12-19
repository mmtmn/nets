use serde::{Serialize, Deserialize};

pub trait System {
    type Observation: Clone;
    type Action: Clone;
    type Score: Ord + Copy;

    fn id(&self) -> String;
    fn seed(&self) -> u64;
    fn reset(&mut self);
    fn step(&mut self, action: Self::Action);
    fn observe(&self) -> Self::Observation;
    fn is_done(&self) -> bool;
    fn score(&self) -> Self::Score;
}

#[derive(Serialize, Deserialize)]
pub struct SystemMeta {
    pub id: String,
    pub deterministic: bool,
    pub verifier_hash: String,
}
