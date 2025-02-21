use objc::{msg_send, sel, sel_impl, class};
use metal::{Device, MTLCreateSystemDefaultDevice};
use super::Platform;
use crate::{Application, RustUI};

pub struct IosPlatform {
    device: Device,
    rust_native: RustUI,
}

impl Platform for IosPlatform {
    fn init(rust_native: RustUI) -> Self {
        let device = unsafe { MTLCreateSystemDefaultDevice() };
        Self { 
            device,
            rust_native,
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
