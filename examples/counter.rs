use rust_ui::*;

struct CounterApp {
    view: View,
}

impl CounterApp {
    fn new() -> Self {
        Self {
            view: View::new()
                .child(Text::new("Count: 0"))
                .child(Button::new("Increment")),
        }
    }
}

impl Application for CounterApp {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn render(&self, renderer: &mut Renderer) {
        self.view.render(renderer);
    }
}

fn main() {
    let rust_ui = RustUI::new();
    rust_ui.run(|| Box::new(CounterApp::new()));
}
