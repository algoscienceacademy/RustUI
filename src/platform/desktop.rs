use crate::{Application, RustUI};
use super::Platform;

pub struct DesktopPlatform {
    rust_ui: RustUI,
}

impl Platform for DesktopPlatform {
    fn init(rust_native: RustUI) -> Self {
        Self { rust_ui: rust_native }
    }

    fn run<F>(self, app: F)
    where
        F: FnOnce() -> Box<dyn Application>
    {
        let mut app = app();
        app.init();
        
        let window = self.rust_ui.window();
        println!("Running desktop app with window size: {:?}", window.dimensions());
        // Implement event loop
    }
}
