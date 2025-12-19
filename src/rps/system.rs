use serde::{Serialize, Deserialize};

use crate::system::System;
use super::r#move::RpsMove;

#[derive(Clone, Serialize, Deserialize)]
pub struct RpsObservation {
    pub round: u32,
}

#[derive(Clone)]
pub struct RpsSystem {
    round: u32,
    max_rounds: u32,
    pending: Option<RpsMove>,
    score: i32,
    done: bool,
}

impl RpsSystem {
    pub fn new(max_rounds: u32) -> Self {
        Self {
            round: 0,
            max_rounds,
            pending: None,
            score: 0,
            done: false,
        }
    }

    fn resolve(a: RpsMove, b: RpsMove) -> i32 {
        use RpsMove::*;
        match (a, b) {
            (x, y) if x == y => 0,
            (Rock, Scissors) => 1,
            (Scissors, Paper) => 1,
            (Paper, Rock) => 1,
            _ => -1,
        }
    }
}

impl System for RpsSystem {
    type Observation = RpsObservation;
    type Action = RpsMove;
    type Score = i32;

    fn reset(&mut self) {
        self.round = 0;
        self.pending = None;
        self.score = 0;
        self.done = false;
    }

    fn observe(&self) -> Self::Observation {
        RpsObservation { round: self.round }
    }

    fn step(&mut self, action: Self::Action) {
        if self.done {
            return;
        }

        if let Some(prev) = self.pending.take() {
            self.score += Self::resolve(prev, action);
            self.round += 1;

            if self.round >= self.max_rounds {
                self.done = true;
            }
        } else {
            self.pending = Some(action);
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn score(&self) -> Self::Score {
        self.score
    }

    fn seed(&self) -> u64 {
        0
    }

    fn id(&self) -> String {
        "rps".into()
    }
}
