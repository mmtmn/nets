use crate::ledger::Ledger;
use crate::league::League;

pub fn settle_league(
    ledger: &mut Ledger,
    league: &League,
    rankings: &[String],
) {
    for (rank, agent_id) in rankings.iter().enumerate() {
        let base_reward = match rank {
            0 => 100,
            1 => 50,
            2 => 25,
            _ => 0,
        };

        let reward = base_reward * league.reward_multiplier;
        ledger.credit(agent_id, reward);
    }
}
