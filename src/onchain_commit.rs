use crate::{
    chain::{MatchCommitment},
    chain_adapter::ChainAdapter,
    league::League,
};

pub fn post_merkle<C: ChainAdapter>(
    chain: &mut C,
    league: &League,
    system_id: &str,
    agent_id: &str,
    merkle_root: [u8; 32],
    score: i64,
) {
    let commitment = MatchCommitment {
        system_id: system_id.into(),
        league_id: league.id.clone(),
        merkle_root,
        agent_ids: vec![agent_id.to_string()],
        scores: vec![score],
    };

    chain.post_commitment(&commitment);
}
