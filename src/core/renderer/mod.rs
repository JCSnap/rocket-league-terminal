use crate::math::{Shape, Point};

pub trait Renderable {
    fn shape(&self) -> Shape;
}

pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn render(&self, renderables: &[&dyn Renderable]) {
        println!("placeholder now, will render renderables");
    }
}

pub trait DrawBackend {
    fn draw_circle(&self, origin: Point, radius: f32);
}
