use crate::math::{Point, Rect};

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Point::default()
        }
    }
}
