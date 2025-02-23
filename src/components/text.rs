use crate::style::Style;
use crate::renderer::Renderer;
use crate::event::Event;
use crate::geometry::Rect;
use super::Component;

pub struct Text {
    content: String,
    style: Style,
    bounds: Rect,
}

impl Text {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            bounds: Rect::default(),
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Component for Text {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.begin_group(&self.style);
        renderer.draw_text(&self.content, &self.style);
        renderer.end_group();
    }

    fn handle_event(&mut self, _event: Event) {}

    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }

    fn apply_style(&mut self, style: Style) {
        self.style = style;
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn style(&self) -> &Style {
        &self.style
    }
}
