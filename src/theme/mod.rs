use std::collections::HashMap;
use crate::style::Style;
use crate::components::Component;

#[derive(Clone)]
pub struct Theme {
    pub is_dark: bool,
    pub colors: Colors,
    styles: HashMap<String, Style>,
}

#[derive(Clone)]
pub struct Colors {
    pub primary: Color,
    pub background: Color,
    pub text: Color,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Theme {
    pub fn new() -> Self {
        Self {
            is_dark: false,
            colors: Colors {
                primary: Color::new(0.0, 0.0, 0.0, 1.0),
                background: Color::new(0.0, 0.0, 0.0, 1.0),
                text: Color::new(0.0, 0.0, 0.0, 1.0),
            },
            styles: HashMap::new(),
        }
    }

    pub fn apply_to_component(&self, component: &mut dyn Component) {
        let style_name = component.style_name();
        if let Some(style) = self.styles.get(style_name) {
            component.apply_style(style.clone());
        }
    }
}

pub fn create_dark_theme() -> Theme {
    Theme {
        is_dark: true,
        colors: Colors {
            primary: Color::new(0.2, 0.6, 1.0, 1.0),
            background: Color::new(0.1, 0.1, 0.1, 1.0),
            text: Color::new(1.0, 1.0, 1.0, 1.0),
        },
        styles: HashMap::new(),
    }
}
