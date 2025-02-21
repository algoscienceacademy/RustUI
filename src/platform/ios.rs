use objc::{msg_send, sel, sel_impl, class};
use metal::Device;
use super::Platform;
use crate::{Application, RustUI};

pub struct IosPlatform {
    device: Device,
    rust_ui: RustUI,
}

impl Platform for IosPlatform {
    fn init(rust_ui: RustUI) -> Self {
        let device = Device::system_default().expect("No Metal device found");
        Self { 
            device,
            rust_ui,
        }
    }

    fn run<F>(self, app: F)
    where
        F: FnOnce() -> Box<dyn Application>
    {
        let mut app = app();
        app.init();
        
        unsafe {
            let _: () = msg_send![class!(UIApplication),
                sharedApplication];
        }
    }
}
