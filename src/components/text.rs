use crate::geometry::{Rect, Point, Size};
use crate::renderer::Renderer;
use crate::event::Event;
use crate::style::Style;
use crate::animation::Animation;
use super::Component;

pub struct Text {
    content: String,
    size: f32,
    bounds: Rect,
}

impl Text {
    pub fn new<S: Into<String>>(content: S) -> Self {
        let mut text = Self {
            content: content.into(),
            size: 14.0,
            bounds: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size { width: 0.0, height: 0.0 },
            },
        };
        text.calculate_size();
        text
    }

    pub fn style(mut self, style: Style) -> Self {
        self.apply_style(style);
        self
    }

    pub fn with_animation(self, _animation: &Animation) -> Self {
        // TODO: Apply animation properties
        self
    }

    fn calculate_size(&mut self) {
        // Basic size calculation based on content length
        let width = self.content.len() as f32 * self.size * 0.6;
        let height = self.size * 1.2;
        self.bounds.size = Size { width, height };
    }
}

impl Component for Text {
    fn render(&self, _renderer: &mut Renderer) {
        // Use content and size for actual rendering
        println!("Rendering text: {} at size {}", self.content, self.size);
    }

    fn handle_event(&mut self, _event: Event) {
        // Implement event handling
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }
}
