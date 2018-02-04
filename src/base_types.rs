pub type GridCord = i16;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GridVector {
    pub x: GridCord,
    pub y: GridCord,
}

#[derive(Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
}
