mod button;
mod text;
mod input;
mod view;
mod stack;

pub use button::Button;
pub use text::Text;
pub use input::Input;
pub use view::View;
pub use stack::{Stack, Direction};

use crate::geometry::Rect;
use crate::renderer::Renderer;
use crate::event::Event;
use crate::style::Style;

pub trait Component {
    fn render(&self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: Event);
    fn bounds(&self) -> Rect;
    fn style_name(&self) -> &str {
        "default"
    }
    fn apply_style(&mut self, _style: Style) {
        // Default implementation does nothing
    }
}
