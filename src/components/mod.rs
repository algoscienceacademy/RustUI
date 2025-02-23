mod button;
mod text;
mod view;

pub use button::Button;
pub use text::Text;
pub use view::View;

use crate::geometry::Rect;
use crate::style::Style;
use crate::event::Event;
use crate::renderer::Renderer;

pub trait Component {
    fn render(&self, renderer: &mut dyn Renderer);
    fn handle_event(&mut self, event: Event);
    fn bounds(&self) -> Rect;
    fn apply_style(&mut self, style: Style);
    fn style_mut(&mut self) -> &mut Style;
    fn style(&self) -> &Style;
    fn style_name(&self) -> &str {
        "default"
    }
}
