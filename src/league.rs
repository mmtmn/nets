use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct League {
    pub id: String,
    pub min_stake: u64,
    pub reward_multiplier: u64,
}

impl League {
    pub fn bronze() -> Self {
        Self {
            id: "bronze".into(),
            min_stake: 0,
            reward_multiplier: 1,
        }
    }

    pub fn silver() -> Self {
        Self {
            id: "silver".into(),
            min_stake: 100,
            reward_multiplier: 2,
        }
    }

    pub fn gold() -> Self {
        Self {
            id: "gold".into(),
            min_stake: 500,
            reward_multiplier: 5,
        }
    }
}

pub fn default_leagues() -> Vec<League> {
    vec![
        League::bronze(),
        League::silver(),
        League::gold(),
    ]
}
