use base_types::*;
use constants::*;
use ggez::{graphics, timer};
use rand::random;
use std::time::Instant;

pub fn since(i: Instant) -> f64 {
    timer::duration_to_f64(i.elapsed())
}

pub fn blinks(time: f64) -> bool {
    time % (BLINK_INTERVAL * 2.0) < BLINK_INTERVAL
}

pub fn modulus(n: GridCord, m: GridCord) -> GridCord {
    ((n % m) + m) % m
}

pub fn wrap_in_grid(gv: GridVector) -> GridVector {
    GridVector {
        x: modulus(gv.x, GRID_SIZE.x),
        y: modulus(gv.y, GRID_SIZE.y),
    }
}

pub fn random_grid_pos() -> GridVector {
    wrap_in_grid(GridVector {
        x: random::<GridCord>(),
        y: random::<GridCord>(),
    })
}

pub fn gv_to_rect(gv: &GridVector) -> graphics::Rect {
    graphics::Rect::new(
        (f32::from(gv.x) * GRID_TILE_SIZE) - GRID_TILE_PADDING,
        (f32::from(gv.y) * GRID_TILE_SIZE) - GRID_TILE_PADDING + INFO_BAR_HIGHT,
        GRID_TILE_SIZE - (GRID_TILE_PADDING * 2.0),
        GRID_TILE_SIZE - (GRID_TILE_PADDING * 2.0),
    )
}

pub fn scale_rect(rect: graphics::Rect, diff: f32) -> graphics::Rect {
    graphics::Rect::new(
        rect.x - diff,
        rect.y - diff,
        rect.w + diff * 2.0,
        rect.h + diff * 2.0,
    )
}
