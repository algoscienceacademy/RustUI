use std::collections::{VecDeque, HashMap};

pub struct Navigator {
    route_stack: VecDeque<Route>,
    transitions: HashMap<(RouteType, RouteType), Box<dyn Transition>>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Route {
    name: String,
    route_type: RouteType,
    params: HashMap<String, String>,
}

impl std::hash::Hash for Route {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.route_type.hash(state);
        // Skip hashing params as HashMap doesn't implement Hash
    }
}

impl Route {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            route_type: RouteType::Push,
            params: HashMap::new(),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum RouteType {
    Push,
    Modal,
    Root,
}

pub trait Transition {
    fn begin(&self);
    fn end(&self);
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            route_stack: VecDeque::new(),
            transitions: HashMap::new(),
        }
    }

    fn get_transition(&self, _from: Option<&RouteType>, _to: Option<&RouteType>) -> Box<dyn Transition> {
        Box::new(DefaultTransition)
    }

    pub fn push(&mut self, route: Route) {
        let transition = self.get_transition(
            self.route_stack.back().map(|r| &r.route_type),
            Some(&route.route_type),
        );
        transition.begin();
        self.route_stack.push_back(route);
    }

    pub fn pop(&mut self) -> Option<Route> {
        let transition = self.get_transition(
            self.route_stack.get(self.route_stack.len() - 2).map(|r| &r.route_type),
            self.route_stack.back().map(|r| &r.route_type),
        );
        transition.begin();
        self.route_stack.pop_back()
    }
}

struct DefaultTransition;

impl Transition for DefaultTransition {
    fn begin(&self) {}
    fn end(&self) {}
}
