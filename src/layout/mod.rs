mod responsive;

pub use responsive::*;
use crate::geometry::Size;

#[derive(Clone, Debug, PartialEq)]
pub enum Layout {
    Row,
    Column,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Column
    }
}

pub const DEFAULT_SPACING: f32 = 8.0;

impl Layout {
    pub fn calculate_size(&self, available_space: Size, spacing: f32) -> Size {
        match self {
            Layout::Row => Size {
                width: available_space.width - spacing,
                height: available_space.height,
            },
            Layout::Column => Size {
                width: available_space.width,
                height: available_space.height - spacing,
            },
        }
    }
}
