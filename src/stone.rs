use base_types::*;
use constants::*;

pub struct Stone {
    pub pos: GridVector,
}

impl Stone {
    pub fn new(x: GridUnit, y: GridUnit) -> Self {
        Self {
            pos: GridVector::new(x, y),
        }
    }
}

pub fn level_corners() -> Vec<Stone> {
    let mut vec = vec![];

    let corners_length = 5;
    let top_left = PLAY_AREA.pos - GridVector::new(1, 1);
    let bottom_right = PLAY_AREA.pos + PLAY_AREA.size;

    vec.push(Stone::new(top_left.x, top_left.y));
    for i in 1..corners_length {
        vec.push(Stone::new(top_left.x + i, top_left.y));
    }
    for i in 1..corners_length {
        vec.push(Stone::new(top_left.x, top_left.y + i));
    }

    vec.push(Stone::new(bottom_right.x, top_left.y));
    for i in 1..corners_length {
        vec.push(Stone::new(bottom_right.x - i, top_left.y));
    }
    for i in 1..corners_length {
        vec.push(Stone::new(bottom_right.x, top_left.y + i));
    }

    vec.push(Stone::new(bottom_right.x, bottom_right.y));
    for i in 1..corners_length {
        vec.push(Stone::new(bottom_right.x - i, bottom_right.y));
    }
    for i in 1..corners_length {
        vec.push(Stone::new(bottom_right.x, bottom_right.y - i));
    }

    vec.push(Stone::new(top_left.x, bottom_right.y));
    for i in 1..corners_length {
        vec.push(Stone::new(top_left.x + i, bottom_right.y));
    }
    for i in 1..corners_length {
        vec.push(Stone::new(top_left.x, bottom_right.y - i));
    }
    vec
}
