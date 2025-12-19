use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChessMove {
    pub from: u8,      // 0..63
    pub to: u8,        // 0..63
    pub promotion: u8 // 0 = none, 1=Q,2=R,3=B,4=N
}

impl ChessMove {
    pub fn new(from: u8, to: u8, promotion: u8) -> Self {
        Self { from, to, promotion }
    }
}
