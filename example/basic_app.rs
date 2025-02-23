use rust_native::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Todo {
    text: String,
    completed: bool,
}

pub struct TodoApp {
    todos: Arc<Mutex<Vec<Todo>>>,
    input: String,
    view: Option<View>,
}

impl TodoApp {
    pub fn new() -> Self {
        let mut app = Self {
            todos: Arc::new(Mutex::new(Vec::new())),
            input: String::new(),
            view: None,
        };
        app.rebuild_ui();
        app
    }

    fn rebuild_ui(&mut self) {
        let todos = self.todos.clone();
        
        let mut main_view = View::new()
            .with_layout(Layout::Column);
        main_view.style_mut().modify().gap = 10.0;

        // Add title
        let mut title = Text::new("RustUI Todo App");
        {
            let style = title.style_mut().modify();
            style.font_size = 24.0;
            style.text_align = TextAlign::Center;
        }
        main_view = main_view.child(title);

        // Add new todo button
        let mut add_button = Button::new("Add Todo");
        {
            let style = add_button.style_mut();
            style.padding = 10.0;
            style.background = Color::rgb(0.2, 0.6, 1.0);
        }

        let todos_ref = todos.clone();
        add_button = add_button.on_click(move || {
            if let Ok(mut todos) = todos_ref.lock() {
                todos.push(Todo::default());
            }
        });

        main_view = main_view.child(add_button);
        main_view = main_view.child(self.build_todo_list());

        self.view = Some(main_view);
    }

    fn build_todo_list(&self) -> View {
        let todos = match self.todos.lock() {
            Ok(guard) => guard,
            Err(_) => return View::new(), // Return empty view on error
        };

        let mut list = View::new()
            .with_layout(Layout::Column);
        list.style_mut().modify().gap = 5.0;

        for todo in todos.iter() {
            // Create checkbox view
            let mut checkbox = Text::new(if todo.completed { "✓" } else { "□" });
            {
                let style = checkbox.style_mut();
                style.color = if todo.completed {
                    Color::rgb(0.0, 0.8, 0.0)
                } else {
                    Color::rgb(0.8, 0.8, 0.8)
                };
            }

            // Create text view
            let mut text = Text::new(&todo.text);
            {
                let style = text.style_mut();
                style.color = if todo.completed {
                    Color::rgb(0.5, 0.5, 0.5)
                } else {
                    Color::rgb(1.0, 1.0, 1.0)
                };
            }

            // Create row container
            let mut row = View::new().with_layout(Layout::Row);
            {
                let style = row.style_mut();
                style.gap = 10.0;
            }

            row = row.child(checkbox).child(text);
            list = list.child(row);
        }

        list
    }
}

impl Application for TodoApp {
    fn init(&mut self) {}

    fn update(&mut self) {
        // For now, just handle window close events
        // You'll need to implement proper event handling based on your Event system
    }

    fn render(&self, renderer: &mut dyn Renderer) {
        renderer.clear(Color::rgb(0.1, 0.1, 0.1));
        if let Some(view) = &self.view {
            view.render(renderer);
        }
    }
}

fn main() {
    RustUI::new().run(|| Box::new(TodoApp::new()));
}
