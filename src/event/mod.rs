use crate::geometry::Point;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    MouseMove(Point),
    MouseDown(Point),
    MouseUp(Point),
    KeyDown(Key),
    KeyUp(Key),
    WindowResize { width: u32, height: u32 },
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Character(char),
    Return,
    Backspace,
    Tab,
    Escape,
    // Add more keys as needed
}

pub struct EventDispatcher {
    handlers: Vec<Box<dyn Fn(&Event)>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, handler: F)
    where
        F: Fn(&Event) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn dispatch(&self, event: Event) {
        for handler in &self.handlers {
            handler(&event);
        }
    }
}
