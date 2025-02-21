use rust_native::{
    Application, RustUI, Renderer,
    components::{Component, View},
};

struct TestApp {
    main_view: View,
}

impl TestApp {
    fn new() -> Self {
        Self {
            main_view: View::new()
        }
    }
}

impl Application for TestApp {
    fn init(&mut self) {
        println!("Application initialized!");
    }

    fn update(&mut self) {}

    fn render(&self, renderer: &mut Renderer) {
        self.main_view.render(renderer);
    }
}

fn main() {
    let rust_native = RustUI::new();
    rust_native.run(|| Box::new(TestApp::new()));
}
