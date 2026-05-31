use crate::math::{Point, Shape, Rect};
use crate::core::{Collider, Renderable};

pub struct Wall {
    collider: Collider
}

impl Wall {
    pub fn new(position: Point, width: f32, height: f32) -> Self {
        Self {
            collider: Collider::new(Shape::Rect(Rect { origin: position, width: width, height: height }))
        }
    }
}

impl Renderable for Wall {
    fn shape(&self) -> Shape {
        self.collider.shape
    }
}
