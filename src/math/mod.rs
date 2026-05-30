#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

pub enum Shape {
    Rect(Rect),
    Circle(Circle)
}

pub struct Rect {
    pub origin: Point,
    pub width: f32,
    pub height: f32
}

pub struct Circle {
    pub origin: Point,
    pub radius: f32
}
