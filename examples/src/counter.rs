use rust_native::*;

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
    let rust_native = RustUI::new();
    rust_native.run(|| Box::new(CounterApp::new()));
}
