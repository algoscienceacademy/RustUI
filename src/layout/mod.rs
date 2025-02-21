mod responsive;

pub use responsive::*;
use crate::geometry::Size;

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

    pub fn calculate_layout(&self, available_space: Size) -> Size {
        match self.direction {
            Direction::Horizontal => Size {
                width: available_space.width - self.spacing,
                height: available_space.height,
            },
            Direction::Vertical => Size {
                width: available_space.width,
                height: available_space.height - self.spacing,
            },
        }
    }
}
