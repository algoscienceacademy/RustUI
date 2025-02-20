use crate::geometry::{Rect, Point, Size};
use crate::renderer::Renderer;
use crate::event::Event;
use super::Component;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct Stack {
    direction: Direction,
    children: Vec<Box<dyn Component>>,
    spacing: f32,
    bounds: Rect,
    child_positions: Vec<Point>,
}

impl Stack {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            children: Vec::new(),
            spacing: 8.0,
            bounds: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size { width: 0.0, height: 0.0 },
            },
            child_positions: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self.recalculate_layout();
        self
    }

    pub fn child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self.recalculate_layout();
        self
    }

    fn recalculate_layout(&mut self) {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;
        self.child_positions.clear();
        
        let mut current_x: f32 = self.bounds.origin.x;
        let mut current_y: f32 = self.bounds.origin.y;

        for child in self.children.iter() {
            let child_bounds = child.bounds();
            
            match self.direction {
                Direction::Horizontal => {
                    self.child_positions.push(Point::new(current_x, current_y));
                    current_x += child_bounds.size.width + self.spacing;
                    width = current_x - self.spacing;
                    height = height.max(child_bounds.size.height);
                }
                Direction::Vertical => {
                    self.child_positions.push(Point::new(current_x, current_y));
                    current_y += child_bounds.size.height + self.spacing;
                    height = current_y - self.spacing;
                    width = width.max(child_bounds.size.width);
                }
            }
        }

        self.bounds.size = Size { width, height };
    }
}

impl Component for Stack {
    fn render(&self, renderer: &mut Renderer) {
        for (child, position) in self.children.iter().zip(self.child_positions.iter()) {
            let mut child_bounds = child.bounds();
            child_bounds.origin = *position;
            // Update child position before rendering
            child.render(renderer);
        }
    }

    fn handle_event(&mut self, event: Event) {
        for child in &mut self.children {
            child.handle_event(event);
        }
    }

    fn bounds(&self) -> Rect {
        self.bounds
    }
}
