use crate::math::{Shape, Circle, Point, Rect};
use crate::core::{Renderable, PhysicsBody, Collider};

pub struct Player {
    pub body: PhysicsBody,
    pub collider: Collider
}

impl Player {
    pub fn new() -> Self {
        let starting_position = Point { x: 0.1, y: 0.1 };
        Self {
            body: PhysicsBody::new(starting_position, 10.0 ),
            collider: Collider::new(Shape::Circle(Circle { origin: starting_position, radius: 5.0 }))
        }
    }
}

impl Renderable for Player {
    fn shape(&self) -> Shape {
        self.collider.shape
    }
}
