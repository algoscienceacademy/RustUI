use std::collections::{VecDeque, HashMap};

pub struct Navigator {
    route_stack: VecDeque<Route>,
    transitions: HashMap<(RouteType, RouteType), Box<dyn Transition>>,
    current_route: Option<RouteType>,
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
            current_route: None,
        }
    }

    pub fn register_transition(&mut self, from: RouteType, to: RouteType, transition: Box<dyn Transition>) {
        self.transitions.insert((from, to), transition);
    }

    fn get_transition(&self, from: Option<&RouteType>, to: Option<&RouteType>) -> Box<dyn Transition> {
        match (from, to) {
            (Some(from), Some(to)) => {
                if let Some(transition) = self.transitions.get(&(from.clone(), to.clone())) {
                    Box::new(DefaultTransition) // For now, return a new instance since we can't clone trait objects
                } else {
                    Box::new(DefaultTransition)
                }
            }
            _ => Box::new(DefaultTransition),
        }
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

    pub fn navigate(&mut self, to: RouteType) {
        if let Some(from) = &self.current_route {
            // Just check for transition existence
            self.transitions.contains_key(&(from.clone(), to.clone()));
        }
        self.current_route = Some(to);
    }
}

#[derive(Clone)]
struct DefaultTransition;

impl Transition for DefaultTransition {
    fn begin(&self) {}
    fn end(&self) {}
}
