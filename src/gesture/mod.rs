use crate::geometry::Point;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum GestureEvent {
    Tap(Point),
    DoubleTap(Point),
    LongPress(Point),
    Pan { start: Point, current: Point },
    Pinch { scale: f32, center: Point },
}

pub struct GestureRecognizer {
    tap_timeout: Duration,
    long_press_timeout: Duration,
    double_tap_timeout: Duration,
    handlers: Vec<Box<dyn Fn(GestureEvent)>>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self {
            tap_timeout: Duration::from_millis(200),
            long_press_timeout: Duration::from_millis(500),
            double_tap_timeout: Duration::from_millis(300),
            handlers: Vec::new(),
        }
    }

    pub fn on_gesture<F>(&mut self, handler: F)
    where
        F: Fn(GestureEvent) + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
}
