#[derive(Clone, Debug, Default)]
pub struct Style {
    pub background: Color,
    pub color: Color,
    pub padding: f32,
    pub margin: f32,
    pub gap: f32,
    pub font_size: f32,
    pub border_radius: f32,
    pub text_align: TextAlign,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn set_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    pub fn set_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn set_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn set_border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn set_text_align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    pub fn get_gap(&self) -> f32 {
        self.gap
    }

    pub fn modify(&mut self) -> &mut Self {
        self
    }
}

impl Color {
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
