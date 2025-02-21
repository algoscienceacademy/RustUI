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

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn get_value(&self) -> &str {
        if self.value.is_empty() {
            &self.placeholder
        } else {
            &self.value
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
