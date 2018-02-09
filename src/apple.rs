use base_types::*;
use std::time::Instant;

pub struct Apple {
    pub position: GridVector,
    pub spawned_at: Instant,
}

impl Apple {
    pub fn new(pos: GridVector) -> Self {
        Apple {
            position: pos,
            spawned_at: Instant::now(),
        }
    }
}
