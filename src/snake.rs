use base_types::*;
use constants::*;
use std::time::Instant;
use utils::random_pos;

#[derive(Debug)]
pub struct Snake {
    pub tail: Vec<GridVector>,
    pub velocity: GridVector,
    pub speed: f64,
    pub last_round: Instant,
    pub lost_at: Option<Instant>,
    pub score: u32,
    pub grow: GridUnit,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            tail: vec![random_pos(&PLAY_AREA)],
            velocity: INITIAL_VELOCITY,
            speed: INITIAL_SPEED,
            last_round: Instant::now(),
            lost_at: None,
            score: 0,
            grow: INITIAL_SIZE - 1,
        }
    }
}
