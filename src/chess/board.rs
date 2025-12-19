use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Piece {
    Empty,
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub squares: Vec<Piece>, // always length 64
    pub white_to_move: bool,
}

impl Board {
    pub fn initial() -> Self {
        use Piece::*;

        let mut s = vec![Empty; 64];

        // White
        s[0]=WR; s[1]=WN; s[2]=WB; s[3]=WQ; s[4]=WK; s[5]=WB; s[6]=WN; s[7]=WR;
        for i in 8..16 { s[i]=WP; }

        // Black
        s[56]=BR; s[57]=BN; s[58]=BB; s[59]=BQ; s[60]=BK; s[61]=BB; s[62]=BN; s[63]=BR;
        for i in 48..56 { s[i]=BP; }

        Self {
            squares: s,
            white_to_move: true,
        }
    }

    pub fn piece(&self, idx: u8) -> Piece {
        self.squares[idx as usize]
    }

    pub fn set(&mut self, idx: u8, p: Piece) {
        self.squares[idx as usize] = p;
    }
}
