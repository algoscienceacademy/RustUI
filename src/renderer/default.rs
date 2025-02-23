use crate::{
    Window,
    style::{Style, Color},
    geometry::{Point, Size},
};
use super::Renderer;

pub struct DefaultRenderer {
    window: Window,
    frame_count: u64,
    transform: Point,
}

impl DefaultRenderer {
    pub fn new(window: &Window) -> Self {
        Self {
            window: window.clone(),
            frame_count: 0,
            transform: Point::default(),
        }
    }

    pub fn present(&mut self) {
        // Implementation for presenting the frame
    }

    pub fn begin_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self) {}

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn get_transform(&self) -> &Point {
        &self.transform
    }

    pub fn get_platform_size(&self) -> Size {
        self.window.get_platform_specific_size()
    }
}

impl Renderer for DefaultRenderer {
    fn clear(&mut self, color: Color) {
        let size = self.get_platform_size();
        // TODO: Implement clear with color and size
        let _ = (color, size); // Temporarily silence warnings
    }

    fn begin_group(&mut self, style: &Style) {
        let size = self.get_platform_size();
        // TODO: Implement begin_group with style and size
        let _ = (style, size); // Temporarily silence warnings
    }

    fn end_group(&mut self) {
        // Implementation coming soon
    }

    fn draw_text(&mut self, _text: &str, _style: &Style) {
        self.frame_count += 1;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.transform.x += x;
        self.transform.y += y;
    }
}
