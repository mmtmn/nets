use crate::system::System;
use crate::agent::Agent;

#[derive(Clone)]
pub struct MatchResult<S: System> {
    pub agent_id: String,
    pub score: S::Score,
}

pub fn run_match<S, A>(
    mut system: S,
    agent: &mut A,
) -> MatchResult<S>
where
    S: System,
    A: Agent<S::Observation, S::Action>,
{
    system.reset();

    while !system.is_done() {
        let obs = system.observe();
        let action = agent.decide(obs);
        system.step(action);
    }

    MatchResult {
        agent_id: agent.id(),
        score: system.score(),
    }
}
