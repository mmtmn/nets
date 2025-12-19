use std::fs;
use serde::{Serialize, Deserialize};

use crate::ledger::Ledger;
use crate::league_state::LeagueState;

#[derive(Serialize, Deserialize, Default)]
pub struct PersistedState {
    pub balances: Vec<(String, u64)>,
    pub disqualified: Vec<String>,

    #[serde(default)]
    pub commitments: Vec<(String, [u8; 32])>, // agent_id -> merkle root
}

pub fn save(
    path: &str,
    ledger: &Ledger,
    league: &LeagueState,
    commitments: Vec<(String, [u8; 32])>,
) {
    let state = PersistedState {
        balances: ledger.snapshot(),
        disqualified: league.disqualified.iter().cloned().collect(),
        commitments,
    };

    fs::write(path, serde_json::to_string_pretty(&state).unwrap()).unwrap();
}

pub fn load(
    path: &str,
    ledger: &mut Ledger,
    league: &mut LeagueState,
) -> PersistedState {
    if let Ok(data) = fs::read_to_string(path) {
        let state: PersistedState = serde_json::from_str(&data).unwrap();

        ledger.restore(state.balances.clone());
        league.disqualified = state.disqualified.clone().into_iter().collect();

        state
    } else {
        PersistedState::default()
    }
}
