use std::sync::Arc;
use crate::style::{Style, Color};
use crate::renderer::Renderer;
use crate::event::Event;
use crate::geometry::Rect;
use super::Component;

type ClickCallback = Arc<dyn Fn() + Send + Sync>;

pub struct Button {
    label: String,
    style: Style,
    bounds: Rect,
    on_click: Option<ClickCallback>,
}

impl Button {
    pub fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            style: Style::default()
                .set_background(Color::rgb(0.2, 0.2, 0.2))
                .set_padding(10.0),
            bounds: Rect::default(),
            on_click: None,
        }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self 
    where
        F: Fn() + Send + Sync + 'static
    {
        self.on_click = Some(Arc::new(callback));
        self
    }
}

impl Component for Button {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.begin_group(&self.style);
        renderer.draw_text(&self.label, &self.style);
        renderer.end_group();
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Click { .. } = event {
            if let Some(callback) = &self.on_click {
                (callback)();
            }
        }
    }

    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }

    fn apply_style(&mut self, style: Style) {
        self.style = style;
    }

    fn style_name(&self) -> &str {
        "button"
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn style(&self) -> &Style {
        &self.style
    }
}
