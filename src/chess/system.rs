use serde::{Serialize, Deserialize};

use crate::system::System;
use super::board::Board;
use super::r#move::ChessMove;
use super::rules::{is_legal, apply, has_king};

#[derive(Clone, Serialize, Deserialize)]
pub struct ChessObservation {
    pub board: Board,
}

#[derive(Clone)]
pub struct ChessSystem {
    board: Board,
    ply: u32,
    max_ply: u32,
    done: bool,
    score: i32,
}

impl ChessSystem {
    pub fn new(max_ply: u32) -> Self {
        Self {
            board: Board::initial(),
            ply: 0,
            max_ply,
            done: false,
            score: 0,
        }
    }
}

impl System for ChessSystem {
    type Observation = ChessObservation;
    type Action = ChessMove;
    type Score = i32;

    fn reset(&mut self) {
        self.board = Board::initial();
        self.ply = 0;
        self.done = false;
        self.score = 0;
    }

    fn observe(&self) -> Self::Observation {
        ChessObservation { board: self.board.clone() }
    }

    fn step(&mut self, action: Self::Action) {
        if self.done { return; }

        if !is_legal(&self.board, action) {
            // illegal move = immediate loss
            self.done = true;
            self.score = if self.board.white_to_move { -1 } else { 1 };
            return;
        }

        apply(&mut self.board, action);
        self.ply += 1;

        if !has_king(&self.board, true) {
            self.done = true;
            self.score = -1;
        } else if !has_king(&self.board, false) {
            self.done = true;
            self.score = 1;
        } else if self.ply >= self.max_ply {
            self.done = true;
            self.score = 0;
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn score(&self) -> Self::Score {
        self.score
    }

    fn seed(&self) -> u64 {
        0 // deterministic chess
    }

    fn id(&self) -> String {
        "chess".into()
    }
}
