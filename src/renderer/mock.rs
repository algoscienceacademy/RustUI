use crate::style::{Style, Color};

pub struct MockRenderer {
    pub(crate) last_color: Option<Color>,
    pub(crate) last_text: Option<String>,
}

impl MockRenderer {
    pub fn new() -> Self {
        Self {
            last_color: None,
            last_text: None,
        }
    }

    pub fn last_color(&self) -> Color {
        self.last_color.unwrap_or(Color::rgb(0.0, 0.0, 0.0))
    }
}

impl super::Renderer for MockRenderer {
    fn clear(&mut self, color: Color) {
        self.last_color = Some(color);
    }

    fn begin_group(&mut self, style: &Style) {
        self.last_color = Some(style.background);
    }

    fn end_group(&mut self) {}

    fn draw_text(&mut self, text: &str, style: &Style) {
        self.last_text = Some(text.to_string());
        self.last_color = Some(style.color);
    }

    fn translate(&mut self, _x: f32, _y: f32) {}
}
