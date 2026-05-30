use crate::math::{Shape, Circle, Point, Rect};
use crate::core::{Renderable, PhysicsBody, Collider};
use crate::constant::{PLAYER_MASS, PLAYER_STARTING_X, PLAYER_STARTING_Y, PLAYER_RADIUS};

pub struct Player {
    pub body: PhysicsBody,
    pub collider: Collider
}

impl Player {
    pub fn new() -> Self {
        let starting_position = Point { x: PLAYER_STARTING_X, y: PLAYER_STARTING_Y };
        Self {
            body: PhysicsBody::new(starting_position, PLAYER_MASS ),
            collider: Collider::new(Shape::Circle(Circle { origin: starting_position, radius: PLAYER_RADIUS }))
        }
    }
}

impl Renderable for Player {
    fn shape(&self) -> Shape {
        Shape::Circle(Circle { origin: self.body.position, radius: PLAYER_RADIUS })
    }
}
