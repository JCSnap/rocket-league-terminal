use crate::math::{Shape, Circle, Point, Vec2};
use crate::constant::GRAVITY;

pub struct Collider {
    pub shape: Shape
}

impl Collider {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape: shape
        }
    }
}

pub struct PhysicsBody {
    position: Point,
    velocity: Vec2,
    force: Vec2,
    mass: f32
}

impl PhysicsBody {
    pub fn new(point: Point, mass: f32) -> Self {
        Self {
            position: point,
            velocity: Vec2 { x: 0.0, y: 0.0 },
            force: Vec2 { x: 0.0, y: 0.0 },
            mass: mass
        }
    }
}

pub struct PhysicsEngine {
    gravity: Vec2
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            gravity: GRAVITY
        }
    }
}
