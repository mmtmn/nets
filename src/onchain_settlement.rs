use crate::{
    chain::{MatchCommitment, SettlementTx},
    chain_adapter::ChainAdapter,
    league::League,
};

pub fn settle_on_chain<C: ChainAdapter>(
    chain: &mut C,
    league: &League,
    system_id: &str,
    rankings: &[(String, i64)],
) {
    let agent_ids: Vec<String> = rankings.iter().map(|r| r.0.clone()).collect();
    let scores: Vec<i64> = rankings.iter().map(|r| r.1).collect();

    let commitment = MatchCommitment {
        system_id: system_id.into(),
        league_id: league.id.clone(),
        merkle_root: [0u8; 32], // supplied from match_trace
        agent_ids: agent_ids.clone(),
        scores,
    };

    let commitment_hash = chain.post_commitment(&commitment);

    let payouts: Vec<(String, u64)> = rankings
        .iter()
        .enumerate()
        .map(|(rank, (id, _))| {
            let base = match rank {
                0 => 100,
                1 => 50,
                2 => 25,
                _ => 0,
            };
            (id.clone(), base * league.reward_multiplier)
        })
        .collect();

    let tx = SettlementTx {
        commitment_hash,
        payouts,
    };

    chain.settle(tx);
}
