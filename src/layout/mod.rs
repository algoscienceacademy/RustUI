mod responsive;

pub use responsive::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Layout {
    direction: Direction,
    spacing: f32,
}

impl Layout {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            spacing: 8.0,
        }
    }
}
