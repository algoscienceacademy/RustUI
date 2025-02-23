pub mod components;
mod renderer;
mod layout;
mod theme;
mod platform;
mod style;
mod event;
pub mod dev_server;  // Make sure this is declared as a module
pub mod window;      // Add window module

// Define geometry module inline to resolve ambiguity
pub mod geometry {
    #[derive(Debug, Clone, Copy)]
    pub struct Size {
        pub width: f32,
        pub height: f32,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Debug, Clone)]
    pub struct Rect {
        pub origin: Point,
        pub size: Size,
    }

    impl Size {
        pub fn new(width: f32, height: f32) -> Self {
            Self { width, height }
        }
    }

    impl Point {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }
    }

    impl Rect {
        pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
            Self {
                origin: Point::new(x, y),
                size: Size::new(width, height),
            }
        }
    }

    impl Default for Size {
        fn default() -> Self {
            Self { width: 800.0, height: 600.0 }
        }
    }

    impl Default for Point {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0 }
        }
    }

    impl Default for Rect {
        fn default() -> Self {
            Self {
                origin: Point::default(),
                size: Size::default(),
            }
        }
    }
}

pub use components::{Button, Text, View, Component};  // Now Component is available at crate root
pub use geometry::{Size, Point, Rect};
pub use event::{Event, KeyCode};
pub use layout::*;
pub use style::{Style, Color, TextAlign};
pub use theme::*;
pub use dev_server::{DevServer, BuildStatus, Platform};
pub use window::Window;  // Export Window type directly
pub use renderer::Renderer;

pub trait Application {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&self, renderer: &mut dyn Renderer);
}

pub struct RustUI {
    window: Window,
    renderer: Box<dyn Renderer>,
}

impl RustUI {
    pub fn new() -> Self {
        let window = Window::new();
        let renderer = Box::new(renderer::DefaultRenderer::new(&window));
        Self { window, renderer }
    }

    pub fn new_with_platform(platform: &str) -> Self {
        let window = Window::new_with_platform(platform);
        let renderer = Box::new(renderer::DefaultRenderer::new(&window));
        Self { window, renderer }
    }

    pub fn run<F>(self, app: F) 
    where
        F: FnOnce() -> Box<dyn Application> + 'static
    {
        platform::run(self, app);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn renderer(&mut self) -> &mut dyn Renderer {
        &mut *self.renderer
    }
}
