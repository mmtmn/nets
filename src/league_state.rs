use std::collections::HashSet;

#[derive(Default)]
pub struct LeagueState {
    pub disqualified: HashSet<String>,
}

impl LeagueState {
    pub fn disqualify(&mut self, agent_id: &str) {
        self.disqualified.insert(agent_id.to_string());
    }

    pub fn is_disqualified(&self, agent_id: &str) -> bool {
        self.disqualified.contains(agent_id)
    }
}
