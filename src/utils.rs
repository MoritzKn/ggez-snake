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

pub fn modulus(n: GridUnit, m: GridUnit) -> GridUnit {
    ((n % m) + m) % m
}

pub fn wrap_in(area: &GridArea, point: GridVector) -> GridVector {
    GridVector {
        x: modulus(point.x - area.pos.x, area.size.x) + area.pos.x,
        y: modulus(point.y - area.pos.x, area.size.y) + area.pos.x,
    }
}

pub fn wrap_in_grid(point: GridVector) -> GridVector {
    wrap_in(&GRID_AREA, point)
}

pub fn random_pos(area: &GridArea) -> GridVector {
    wrap_in(
        &area,
        GridVector {
            x: random::<GridUnit>(),
            y: random::<GridUnit>(),
        },
    )
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
