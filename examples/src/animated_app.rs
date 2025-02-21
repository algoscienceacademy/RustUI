use rust_native::*;
use std::time::Duration;

struct AnimatedApp {
    view: View,
}

impl Application for AnimatedApp {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn render(&self, renderer: &mut Renderer) {
        self.view.render(renderer);
    }
}

fn main() {
    let rust_native = RustUI::new();
    let theme = create_dark_theme();
    
    rust_native.run(|| {
        let animation = animation::Animation::new(0.0, 1.0, Duration::from_secs(1))
            .with_easing(animation::EasingFunction::EaseInOut);
            
        let view = View::new()
            .with_theme(theme)
            .with_gesture_recognizer(|_recognizer| {})
            .child(Text::new("Animated Text").with_animation(&animation));

        Box::new(AnimatedApp { view })
    });
}
