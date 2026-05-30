mod ratatui_backend;
pub use ratatui_backend::RatatuiBackend;

use crossterm::terminal::size;
use crate::math::{Shape, Circle, Point};

pub trait Renderable {
    fn shape(&self) -> Shape;
}

pub struct Renderer {
    backend: Box<dyn DrawBackend>
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            backend: Box::new(RatatuiBackend::new())
        }
    }

    pub fn render(&mut self, renderables: &[&dyn Renderable]) {
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));
        self.backend.render(&renderables, width, height);
    }
}

pub trait DrawBackend {
    fn render(&mut self, renderables: &[&dyn Renderable], width: u16, height: u16);
}
