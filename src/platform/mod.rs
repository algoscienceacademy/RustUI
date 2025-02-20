use crate::{Application, RustUI};

#[cfg(target_os = "ios")]
mod ios;
#[cfg(target_os = "android")]
mod android;
#[cfg(target_arch = "wasm32")]
mod web;
mod desktop;

pub trait Platform {
    fn init(rust_ui: RustUI) -> Self;
    fn run<F>(self, app: F) where F: FnOnce() -> Box<dyn Application>;
}

#[cfg(target_os = "ios")]
pub use ios::IosPlatform as CurrentPlatform;
#[cfg(target_os = "android")]
pub use android::AndroidPlatform as CurrentPlatform;
#[cfg(target_arch = "wasm32")]
pub use web::WebPlatform as CurrentPlatform;
#[cfg(not(any(target_os = "ios", target_os = "android", target_arch = "wasm32")))]
pub use desktop::DesktopPlatform as CurrentPlatform;

pub fn run<F>(rust_ui: RustUI, app: F)
where
    F: FnOnce() -> Box<dyn Application>
{
    let platform = CurrentPlatform::init(rust_ui);
    platform.run(app);
}
