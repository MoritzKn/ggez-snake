pub type GridUnit = i16;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GridVector {
    pub x: GridUnit,
    pub y: GridUnit,
}

#[derive(Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
}
