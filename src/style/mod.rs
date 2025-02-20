use std::collections::HashMap;

#[derive(Clone)]
pub struct Style {
    properties: HashMap<String, StyleValue>,
}

#[derive(Clone)]
pub enum StyleValue {
    Number(f32),
    Color(crate::theme::Color),
    String(String),
}

impl From<f32> for StyleValue {
    fn from(value: f32) -> Self {
        StyleValue::Number(value)
    }
}

impl From<f64> for StyleValue {
    fn from(value: f64) -> Self {
        StyleValue::Number(value as f32)
    }
}

impl From<crate::theme::Color> for StyleValue {
    fn from(value: crate::theme::Color) -> Self {
        StyleValue::Color(value)
    }
}

impl From<String> for StyleValue {
    fn from(value: String) -> Self {
        StyleValue::String(value)
    }
}

impl From<&str> for StyleValue {
    fn from(value: &str) -> Self {
        StyleValue::String(value.to_string())
    }
}

impl Style {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn set<T: Into<StyleValue>>(mut self, key: &str, value: T) -> Self {
        self.properties.insert(key.to_string(), value.into());
        self
    }
}
