use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

use crate::system::System;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct SnakeSystem {
    width: i32,
    height: i32,

    snake: VecDeque<(i32, i32)>,
    dir: Dir,
    apple: (i32, i32),

    step: u64,
    max_steps: u64,
    done: bool,
    score: i64,
}

impl SnakeSystem {
    pub fn new(width: i32, height: i32, max_steps: u64) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((width / 2, height / 2));

        let apple = Self::spawn_apple(0, width, height, &snake);

        Self {
            width,
            height,
            snake,
            dir: Dir::Right,
            apple,
            step: 0,
            max_steps,
            done: false,
            score: 0,
        }
    }

    fn spawn_apple(
        step: u64,
        width: i32,
        height: i32,
        snake: &VecDeque<(i32, i32)>,
    ) -> (i32, i32) {
        let mut x = (step as i32 * 7 + 13) % width;
        let mut y = (step as i32 * 11 + 29) % height;

        while snake.contains(&(x, y)) {
            x = (x + 1) % width;
            y = (y + 1) % height;
        }

        (x, y)
    }

    fn next_head(&self) -> (i32, i32) {
        let (x, y) = *self.snake.front().unwrap();

        match self.dir {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        }
    }
}

impl System for SnakeSystem {
    type Observation = (
        (i32, i32),
        (i32, i32),
        Vec<(i32, i32)>,
    );

    type Action = Dir;
    type Score = i64;

    fn id(&self) -> String {
        "snake_v1".into()
    }

    fn seed(&self) -> u64 {
        0
    }

    fn reset(&mut self) {
        *self = Self::new(self.width, self.height, self.max_steps);
    }

    fn observe(&self) -> Self::Observation {
        (
            *self.snake.front().unwrap(),
            self.apple,
            self.snake.iter().cloned().collect(),
        )
    }

    fn step(&mut self, action: Self::Action) {
        if self.done {
            return;
        }

        self.dir = action;
        let next = self.next_head();

        if next.0 < 0
            || next.1 < 0
            || next.0 >= self.width
            || next.1 >= self.height
            || self.snake.contains(&next)
        {
            self.done = true;
            return;
        }

        self.snake.push_front(next);

        if next == self.apple {
            self.score += 10;
            self.apple = Self::spawn_apple(
                self.step,
                self.width,
                self.height,
                &self.snake,
            );
        } else {
            self.snake.pop_back();
        }

        self.step += 1;

        if self.step >= self.max_steps {
            self.done = true;
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn score(&self) -> Self::Score {
        self.score + self.snake.len() as i64
    }
}
