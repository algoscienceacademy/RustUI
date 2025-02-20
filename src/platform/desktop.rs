use crate::{Application, RustUI};
use super::Platform;

pub struct DesktopPlatform {
    rust_ui: RustUI,
}

impl Platform for DesktopPlatform {
    fn init(rust_ui: RustUI) -> Self {
        Self { rust_ui }
    }

    fn run<F>(self, app: F)
    where
        F: FnOnce() -> Box<dyn Application>
    {
        let mut app = app();
        app.init();
        // Desktop event loop implementation
    }
}
