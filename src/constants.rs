use base_types::*;
use ggez::graphics::Color;

//--- Grid ---//
pub const GRID_TILE_SIZE: f32 = 20.0;
pub const GRID_SIZE: GridVector = GridVector { x: 40, y: 22 };
pub static GRID_AREA: GridArea = GridArea {
    pos: GridVector { x: 0, y: 0 },
    size: GRID_SIZE,
};
pub static PLAY_AREA: GridArea = GridArea {
    pos: GridVector { x: 2, y: 2 },
    size: GridVector {
        x: GRID_SIZE.x - 4,
        y: GRID_SIZE.y - 4,
    },
};

//--- Visual ---//
pub const APPLE_BLINK_TIME: f64 = 0.7;
pub const BLINK_INTERVAL: f64 = 0.08;
pub const BLINK_LENGTH_AFTER_DEATH: f64 = 0.6;
pub const COLOR_BACKGROUND: Color = Color {
    r: 0.0078,
    g: 0.0569,
    b: 0.0762,
    a: 1.0,
};
pub const COLOR_FOREGROUND: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub const FONT_DEFAULT_SIZE: u32 = 22;
pub const FONT_GAME_OVER_SIZE: u32 = 80;
pub const GAME_OVER_TIMEOUT: f64 = 0.9;
pub const GRID_TILE_PADDING: f32 = 1.0;
pub const INFO_BAR_HIGHT: f32 = 40.0;

//--- Game play ---//
pub const INITIAL_SIZE: GridUnit = 2;
pub const INITIAL_SPEED: f64 = 0.1;
pub const INITIAL_VELOCITY: GridVector = GridVector { x: 1, y: 0 };

pub const GROW_PER_APPLE: GridUnit = 3;
pub const SPEED_INCREASE_FRACTION: f64 = 36.0;
