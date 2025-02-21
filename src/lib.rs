pub mod components;
pub mod platform;
pub mod renderer;
pub mod animation;
pub mod gesture;
pub mod layout;
pub mod navigation;
pub mod event;
pub mod geometry;
pub mod store;
pub mod theme;
pub mod style;
pub mod dev_server;

pub use components::*;
pub use platform::*;
pub use renderer::*;
pub use event::*;
pub use geometry::*;
pub use store::Store;
pub use theme::*;
pub use dev_server::{BuildStatus, DevServer, Platform};

pub trait Application {
    fn init(&mut self);
    fn update(&mut self);
    fn render(&self, renderer: &mut Renderer);
}

#[derive(Clone)]
pub struct Window {
    width: u32,
    height: u32,
}

impl Window {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub struct RustUI {
    window: Window,
    renderer: Renderer,
}

impl RustUI {
    pub fn new() -> Self {
        let window = Window::new();
        let renderer = Renderer::new(&window);
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

    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}
