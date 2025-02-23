use crate::geometry::Point;
use std::sync::mpsc::Receiver;
use crossterm::event::{self as term_event, Event as TermEvent, KeyEvent};

pub fn poll() -> Option<Event> {
    static mut EVENT_RECEIVER: Option<Receiver<Event>> = None;
    
    if let Ok(true) = term_event::poll(std::time::Duration::from_millis(100)) {
        if let Ok(TermEvent::Key(key)) = term_event::read() {
            return Some(Event::from_key_event(key));
        }
    }
    None
}

#[derive(Clone, Debug)]
pub enum Event {
    KeyPress(KeyCode),
    Click { x: f32, y: f32 },
    MouseMove(Point),
    TouchStart(Point),
    TouchEnd(Point),
}

impl Event {
    fn from_key_event(key: KeyEvent) -> Self {
        use crossterm::event::KeyCode as TermKeyCode;
        let code = match key.code {
            TermKeyCode::Char('q') => KeyCode::Q,
            TermKeyCode::Up => KeyCode::Up,
            TermKeyCode::Down => KeyCode::Down,
            TermKeyCode::Left => KeyCode::Left,
            TermKeyCode::Right => KeyCode::Right,
            _ => return Event::KeyPress(KeyCode::Q),
        };
        Event::KeyPress(code)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    Q,
    Plus,
    Minus,
}
