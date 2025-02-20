use crate::geometry::{Rect, Point, Size};
use crate::renderer::Renderer;
use crate::event::Event;
use super::Component;

pub struct Input {
    value: String,
    placeholder: String,
    bounds: Rect,
}

impl Input {
    pub fn new<S: Into<String>>(placeholder: S) -> Self {
        Self {
            value: String::new(),
            placeholder: placeholder.into(),
            bounds: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size { width: 200.0, height: 40.0 },
            },
        }
    }
}

impl Component for Input {
    fn render(&self, _renderer: &mut Renderer) {
        // Implement rendering
    }

    fn handle_event(&mut self, _event: Event) {
        // Implement event handling
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }
}
