use crate::{
    system::System,
    agent::Agent,
    r#match::{run_match, MatchResult},
};

pub fn round_robin<S, A>(
    system: S,
    agents: &mut [A],
) -> Vec<MatchResult<S>>
where
    S: System + Clone,
    A: Agent<S::Observation, S::Action>,
{
    let mut results = Vec::new();

    for agent in agents.iter_mut() {
        let result = run_match(system.clone(), agent);
        results.push(result);
    }

    results
}
