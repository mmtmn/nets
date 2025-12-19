use sha2::{Sha256, Digest};
use serde::Serialize;

use crate::{
    system::System,
    agent::Agent,
    trace::TraceStep,
    merkle::MerkleTree,
};

fn hash_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().into()
}

fn hash_val<T: Serialize>(v: &T) -> [u8; 32] {
    hash_bytes(&bincode::serialize(v).unwrap())
}

pub struct MatchTrace {
    pub steps: Vec<TraceStep>,
    pub merkle: MerkleTree,
}

pub fn run_match_with_trace<S, A>(
    mut system: S,
    agent: &mut A,
) -> MatchTrace
where
    S: System,
    S::Observation: Serialize,
    S::Action: Serialize,
    A: Agent<S::Observation, S::Action>,
{
    let mut steps = Vec::new();
    system.reset();

    let mut i = 0u64;

    while !system.is_done() {
        let obs = system.observe();
        let obs_hash = hash_val(&obs);

        let action = agent.decide(obs);
        let action_hash = hash_val(&action);

        steps.push(TraceStep {
            step: i,
            obs_hash,
            action_hash,
        });

        system.step(action);
        i += 1;
    }

    let leaves = steps.iter().map(|s| s.hash()).collect();
    let merkle = MerkleTree::build(leaves);

    MatchTrace { steps, merkle }
}
