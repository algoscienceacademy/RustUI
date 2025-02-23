use crate::style::Style;
use crate::layout::Layout;
use crate::renderer::Renderer;
use crate::event::Event;
use crate::geometry::Rect;
use super::Component;

pub struct View {
    children: Vec<Box<dyn Component>>,
    style: Style,
    layout: Layout,
    bounds: Rect,
}

impl View {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::default(),
            layout: Layout::Column,
            bounds: Rect::default(),
        }
    }

    pub fn child<C: Component + 'static>(mut self, component: C) -> Self {
        self.children.push(Box::new(component));
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl Component for View {
    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.begin_group(&self.style);
        
        match self.layout {
            Layout::Row => {
                let mut x = 0.0;
                for child in &self.children {
                    renderer.translate(x, 0.0);
                    child.render(renderer);
                    x += self.style.get_gap();
                }
            }
            Layout::Column => {
                let mut y = 0.0;
                for child in &self.children {
                    renderer.translate(0.0, y);
                    child.render(renderer);
                    y += self.style.get_gap();
                }
            }
        }
        
        renderer.end_group();
    }

    fn handle_event(&mut self, event: Event) {
        for child in &mut self.children {
            child.handle_event(event.clone());
        }
    }

    fn bounds(&self) -> Rect {
        self.bounds.clone()
    }

    fn apply_style(&mut self, style: Style) {
        self.style = style;
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}
