use crate::geometry::Size;

#[derive(Clone)]
pub struct Window {
    width: u32,
    height: u32,
    scale_factor: f32,
    platform: PlatformWindow,
}

#[derive(Clone)]
enum PlatformWindow {
    Desktop(DesktopWindow),
    Mobile(MobileWindow),
    Web(WebWindow),
}

#[derive(Clone)]
struct DesktopWindow {
    min_size: Size,
    max_size: Size,
    resizable: bool,
}

#[derive(Clone)]
struct MobileWindow {
    safe_area: SafeArea,
    orientation: Orientation,
}

#[derive(Clone)]
struct WebWindow {
    container_id: String,
}

#[derive(Clone, Default)]
pub struct SafeArea {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Clone, Copy)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Window {
    pub fn new() -> Self {
        Self::new_with_platform("desktop")
    }

    pub fn new_with_platform(platform: &str) -> Self {
        let platform = match platform {
            "desktop" => PlatformWindow::Desktop(DesktopWindow {
                min_size: Size::new(300.0, 200.0),
                max_size: Size::new(f32::MAX, f32::MAX),
                resizable: true,
            }),
            "ios" | "android" => PlatformWindow::Mobile(MobileWindow {
                safe_area: SafeArea::default(),
                orientation: Orientation::Portrait,
            }),
            "web" => PlatformWindow::Web(WebWindow {
                container_id: "rust-ui-app".to_string(),
            }),
            _ => PlatformWindow::Desktop(DesktopWindow {
                min_size: Size::new(300.0, 200.0),
                max_size: Size::new(f32::MAX, f32::MAX),
                resizable: true,
            }),
        };

        Self {
            width: 800,
            height: 600,
            scale_factor: 1.0,
            platform,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_platform_specific_size(&self) -> Size {
        Size::new(self.width as f32, self.height as f32)
    }

    pub fn get_scale_factor(&self) -> f32 {
        self.scale_factor
    }
}
