use crate::style::{Style, Color};

mod default;
#[cfg(test)]
mod mock;

pub use default::DefaultRenderer;

pub trait Renderer {
    fn clear(&mut self, color: Color);
    fn begin_group(&mut self, style: &Style);
    fn end_group(&mut self);
    fn draw_text(&mut self, text: &str, style: &Style);
    fn translate(&mut self, x: f32, y: f32);
}
