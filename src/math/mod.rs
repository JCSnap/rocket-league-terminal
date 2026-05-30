#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

#[derive(Copy, Clone)]
pub enum Shape {
    Circle(Circle)
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub origin: Point,
    pub width: f32,
    pub height: f32
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub origin: Point,
    pub radius: f32
}
