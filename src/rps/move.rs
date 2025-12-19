use serde::{Serialize, Deserialize};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RpsMove {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl RpsMove {
    pub fn from_u64(v: u64) -> Option<Self> {
        match v % 3 {
            0 => Some(RpsMove::Rock),
            1 => Some(RpsMove::Paper),
            2 => Some(RpsMove::Scissors),
            _ => None,
        }
    }
}
