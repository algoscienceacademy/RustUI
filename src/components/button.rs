use crate::geometry::{Rect, Point, Size};
use crate::renderer::Renderer;
use crate::event::Event;
use super::Component;

pub struct Button {
    label: String,
    on_click: Box<dyn Fn()>,
    bounds: Rect,
}

impl Button {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            on_click: Box::new(|| {}),
            bounds: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size { width: 100.0, height: 40.0 },
            },
        }
    }

    pub fn on_click<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.on_click = Box::new(f);
        self
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.bounds.origin.x 
            && point.x <= self.bounds.origin.x + self.bounds.size.width
            && point.y >= self.bounds.origin.y 
            && point.y <= self.bounds.origin.y + self.bounds.size.height
    }
}

impl Component for Button {
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
