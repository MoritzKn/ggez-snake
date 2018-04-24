use std::ops;

pub type GridUnit = i16;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GridVector {
    pub x: GridUnit,
    pub y: GridUnit,
}

impl GridVector {
    pub fn new(x: GridUnit, y: GridUnit) -> Self {
        Self { x, y }
    }
}

impl ops::Add<GridVector> for GridVector {
    type Output = GridVector;

    fn add(self, rhs: GridVector) -> GridVector {
        GridVector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<GridVector> for GridVector {
    type Output = GridVector;

    fn sub(self, rhs: GridVector) -> GridVector {
        GridVector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub struct GridArea {
    pub pos: GridVector,
    pub size: GridVector,
}
