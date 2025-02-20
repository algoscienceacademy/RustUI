use rust_ui::{
    Application, RustUI, Renderer,
    components::*,
    style::Style,
    theme::Color,
};

struct StyledApp {
    view: View,
}

impl Application for StyledApp {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn render(&self, renderer: &mut Renderer) {
        self.view.render(renderer);
    }
}

fn main() {
    let rust_ui = RustUI::new();
    
    rust_ui.run(|| {
        let view = View::new()
            .child(
                Text::new("Welcome")
                    .style(Style::new()
                        .set("font-size", 24.0)
                        .set("color", Color::new(0.1, 0.1, 0.1, 1.0)))
            )
            .child(
                Stack::new(Direction::Horizontal)
                    .spacing(8.0)
                    .child(Button::new("Login"))
                    .child(Button::new("Register"))
            );
            
        Box::new(StyledApp { view })
    });
}
