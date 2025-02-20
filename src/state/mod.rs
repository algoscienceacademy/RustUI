use std::sync::{Arc, Mutex};
use std::any::Any;

pub struct Store<T: 'static> {
    state: Arc<Mutex<T>>,
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T: Clone + 'static> Store<T> {
    pub fn new(initial_state: T) -> Self {
        Self {
            state: Arc::new(Mutex::new(initial_state)),
            subscribers: Vec::new(),
        }
    }

    pub fn dispatch<A>(&mut self, action: A)
    where
        A: FnOnce(&mut T),
    {
        let mut state = self.state.lock().unwrap();
        action(&mut state);
        for subscriber in &self.subscribers {
            subscriber(&state);
        }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&T) + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }
}
