use crate::{
    system::System,
    agent::Agent,
    league::League,
    eligibility::can_enter,
    r#match::{run_match, MatchResult},
    ledger::Ledger,
};

pub struct LeagueConfig {
    pub matches_per_agent: usize,
}

pub struct LeagueResult<S: System> {
    pub agent_id: String,
    pub total_score: i64,
    pub matches: Vec<MatchResult<S>>,
}

pub fn run_league<S, A>(
    system: S,
    agents: &mut [A],
    ledger: &Ledger,
    league: &League,
    cfg: &LeagueConfig,
) -> Vec<LeagueResult<S>>
where
    S: System + Clone,
    A: Agent<S::Observation, S::Action>,
{
    let mut results = Vec::new();

    for agent in agents.iter_mut() {
        if !can_enter(ledger, &agent.id(), league) {
            continue;
        }

        let mut matches = Vec::new();
        let mut total = 0i64;

        for _ in 0..cfg.matches_per_agent {
            let r = run_match(system.clone(), agent);
            total += r.score as i64;
            matches.push(r);
        }

        results.push(LeagueResult {
            agent_id: agent.id(),
            total_score: total,
            matches,
        });
    }

    results
}
