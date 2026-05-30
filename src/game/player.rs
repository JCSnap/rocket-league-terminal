use crate::math::{Shape, Circle, Point, Rect};
use crate::core::{Renderable};

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Point { x: 0.1, y: 0.1 }
        }
    }
}

impl Renderable for Player {
    fn shape(&self) -> Shape {
        Shape::Circle( Circle { origin: self.position, radius: 5.0 } )
    }
}
