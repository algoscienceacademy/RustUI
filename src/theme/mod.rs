use std::collections::HashMap;
use crate::style::Style;
use crate::components::Component;

#[derive(Clone)]
pub struct Theme {
    pub is_dark: bool,
    pub colors: ThemeColors,
    styles: HashMap<String, Style>,
}

#[derive(Clone)]
pub struct ThemeColors {
    pub primary: crate::style::Color,
    pub background: crate::style::Color,
    pub text: crate::style::Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            is_dark: false,
            colors: ThemeColors {
                primary: crate::style::Color::rgb(0.0, 0.0, 0.0),
                background: crate::style::Color::rgb(1.0, 1.0, 1.0),
                text: crate::style::Color::rgb(0.0, 0.0, 0.0),
            },
            styles: HashMap::new(),
        }
    }

    pub fn apply_to(&self, component: &mut dyn Component) {
        if let Some(style) = self.styles.get("default") {
            component.apply_style(style.clone());
        }
    }
}

pub fn create_dark_theme() -> Theme {
    Theme {
        is_dark: true,
        colors: ThemeColors {
            primary: crate::style::Color::rgb(0.2, 0.6, 1.0),
            background: crate::style::Color::rgb(0.1, 0.1, 0.1),
            text: crate::style::Color::rgb(1.0, 1.0, 1.0),
        },
        styles: HashMap::new(),
    }
}
