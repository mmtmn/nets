use std::fs;
use serde::{Serialize, Deserialize};

use crate::ledger::Ledger;
use crate::league_state::LeagueState;

#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub balances: Vec<(String, u64)>,
    pub disqualified: Vec<String>,
}

pub fn save(path: &str, ledger: &Ledger, league: &LeagueState) {
    let state = PersistedState {
        balances: ledger.snapshot(),
        disqualified: league.disqualified.iter().cloned().collect(),
    };

    fs::write(path, serde_json::to_string_pretty(&state).unwrap()).unwrap();
}

pub fn load(path: &str, ledger: &mut Ledger, league: &mut LeagueState) {
    if let Ok(data) = fs::read_to_string(path) {
        let state: PersistedState = serde_json::from_str(&data).unwrap();

        ledger.restore(state.balances);
        league.disqualified = state.disqualified.into_iter().collect();
    }
}
