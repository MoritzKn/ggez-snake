use base_types::*;
use ggez::graphics::Color;

//--- Grid ---//
pub const GRID_SIZE: GridVector = GridVector { x: 30, y: 20 };
pub const GRID_TILE_SIZE: f32 = 20.0;

//--- Visual ---//
pub const APPLE_BLINK_TIME: f64 = 0.5;
pub const BLINK_INTERVAL: f64 = 0.08;
pub const BLINK_LENGTH_AFTER_DEATH: f64 = 0.6;
pub const COLOR_BACKGROUND: Color = Color {
    r: 0.0078,
    g: 0.0529,
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
pub const GAME_OVER_TIMEOUT: f64 = 0.5;
pub const GRID_TILE_PADDING: f32 = 1.0;
pub const INFO_BAR_HIGHT: f32 = 40.0;

//--- Game play ---//
pub const INITIAL_SIZE: GridCord = 1;
pub const INITIAL_SPEED: f64 = 0.06;
pub const INITIAL_VELOCITY: GridVector = GridVector { x: 0, y: 0 };

pub const GROW_PER_APPLE: GridCord = 2;
pub const SPEED_INCREASE_FRACTION: f64 = 26.0;
