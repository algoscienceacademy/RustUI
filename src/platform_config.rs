use crate::geometry::Size;

pub struct PlatformConfig {
    pub window_size: Size,
    pub scale_factor: f32,
    pub default_padding: f32,
    pub font_size_multiplier: f32,
}

impl PlatformConfig {
    pub fn for_platform(platform: &str) -> Self {
        match platform {
            "ios" => Self {
                window_size: Size::new(390.0, 844.0), // iPhone 15 size
                scale_factor: 2.0,
                default_padding: 20.0,
                font_size_multiplier: 1.2,
            },
            "android" => Self {
                window_size: Size::new(411.0, 869.0), // Pixel 4 size
                scale_factor: 2.75,
                default_padding: 16.0,
                font_size_multiplier: 1.0,
            },
            "web" => Self {
                window_size: Size::new(800.0, 600.0),
                scale_factor: 1.0,
                default_padding: 12.0,
                font_size_multiplier: 1.0,
            },
            _ => Self {
                window_size: Size::new(800.0, 600.0),
                scale_factor: 1.0,
                default_padding: 16.0,
                font_size_multiplier: 1.0,
            },
        }
    }
}
