use std::ops::{AddAssign, Div, Mul};

#[derive(Copy, Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn clamp_components(&self, max_mag: f32) -> Vec2 {
        Vec2 {
            x: self.x.clamp(-max_mag, max_mag),
            y: self.y.clamp(-max_mag, max_mag)
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
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
