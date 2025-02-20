use std::cell::RefCell;
use std::rc::Rc;

pub struct Store<T> {
    state: Rc<RefCell<T>>,
}

impl<T: Clone> Store<T> {
    pub fn new(initial_state: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(initial_state)),
        }
    }

    pub fn dispatch<F>(&self, mutation: F)
    where
        F: FnOnce(&mut T),
    {
        let mut state = self.state.borrow_mut();
        mutation(&mut state);
    }

    pub fn get_state(&self) -> T {
        self.state.borrow().clone()
    }
}
