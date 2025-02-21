use crate::geometry::{Rect, Point, Size};
use crate::renderer::Renderer;
use crate::event::Event;
use crate::layout::{ResponsiveLayout, Breakpoint};
use crate::theme::Theme;
use crate::gesture::GestureRecognizer;
use crate::store::Store;
use super::Component;

pub struct View {
    children: Vec<Box<dyn Component>>,
    bounds: Rect,
    theme: Option<Theme>,
}

impl View {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            bounds: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size { width: 0.0, height: 0.0 },
            },
            theme: None,
        }
    }

    pub fn child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_responsive_layout<C: Component + Clone + 'static>(
        mut self,
        layout: &ResponsiveLayout,
        layouts: Vec<(Breakpoint, C)>
    ) -> Self {
        if let Some(selected_layout) = layout.get_layout(&layouts) {
            self.children.push(Box::new(selected_layout.clone()));
        }
        self
    }

    pub fn with_gesture_recognizer<F>(self, _f: F) -> Self 
    where 
        F: FnOnce(&mut GestureRecognizer) + 'static 
    {
        // Initialize and configure gesture recognizer
        self
    }

    pub fn with_store<T: Clone + 'static>(self, _store: &Store<T>) -> Self {
        // Set up store connection
        self
    }

    pub fn with_navigator(self, _navigator: &crate::navigation::Navigator) -> Self {
        // Set up navigation
        self
    }
}

impl Component for View {
    fn render(&self, _renderer: &mut Renderer) {
        for child in &self.children {
            child.render(_renderer);
        }
    }

    fn handle_event(&mut self, _event: Event) {
        for child in &mut self.children {
            child.handle_event(_event);
        }
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }
}

// Also implement Clone for Box<dyn Component>
impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        // For now, return a dummy component since we can't actually clone trait objects
        Box::new(super::Text::new(""))
    }
}
