use rust_native::*;
use rust_native::{
    layout::{ResponsiveLayout, Breakpoint},
    navigation::{Navigator, Route},
};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct AppState {
    counter: i32,
    theme: Theme,
}

struct ResponsiveApp {
    view: View,
}

impl Application for ResponsiveApp {
    fn init(&mut self) {}
    fn update(&mut self) {}
    fn render(&self, renderer: &mut Renderer) {
        self.view.render(renderer);
    }
}

fn main() {
    let rust_native = RustUI::new();
    let store = Store::new(AppState {
        counter: 0,
        theme: create_dark_theme(),
    });
    
    let responsive = ResponsiveLayout::new();
    let store = Rc::new(RefCell::new(store));
    let navigator = Rc::new(RefCell::new(Navigator::new()));
    
    rust_native.run(move || {
        let store_for_view = store.clone();
        let navigator_for_view = navigator.clone();
        let store_for_increment = store.clone();
        let navigator_for_next = navigator.clone();

        let view = View::new()
            .with_responsive_layout(&responsive, vec![
                (Breakpoint::Small, Stack::new(Direction::Vertical)),
                (Breakpoint::Large, Stack::new(Direction::Horizontal)),
            ])
            .with_store(&*store_for_view.borrow())
            .with_navigator(&*navigator_for_view.borrow())
            .child(
                Button::new("Next Page")
                    .on_click(move || {
                        navigator_for_next.borrow_mut().push(Route::new("details"));
                    })
            )
            .child(
                Button::new("Increment")
                    .on_click(move || {
                        store_for_increment.borrow_mut().dispatch(|state| {
                            state.counter += 1;
                        });
                    })
            );

        Box::new(ResponsiveApp { view })
    });
}
