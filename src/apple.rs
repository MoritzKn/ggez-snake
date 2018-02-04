use base_types::*;
use std::time::Instant;
use utils::random_grid_pos;

pub struct Apple {
    pub position: GridVector,
    pub spawned_at: Instant,
}

impl Apple {
    pub fn new() -> Self {
        Apple {
            position: random_grid_pos(),
            spawned_at: Instant::now(),
        }
    }
}
