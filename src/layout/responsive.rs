#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Breakpoint {
    Small,    // 0-600px
    Medium,   // 600-960px
    Large,    // 960-1280px
    XLarge,   // 1280px+
}

#[allow(dead_code)]
pub struct ResponsiveLayout {
    breakpoints: Vec<(f32, Breakpoint)>,
    current: Breakpoint,
}

impl ResponsiveLayout {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![
                (600.0, Breakpoint::Small),
                (960.0, Breakpoint::Medium),
                (1280.0, Breakpoint::Large),
                (f32::MAX, Breakpoint::XLarge),
            ],
            current: Breakpoint::Medium,
        }
    }

    pub fn get_layout<'a, T>(&self, layouts: &'a [(Breakpoint, T)]) -> Option<&'a T> {
        layouts.iter()
            .find(|(bp, _)| self.current == *bp)
            .map(|(_, layout)| layout)
    }
}
