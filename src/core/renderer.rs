use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use crate::math::Shape;

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
}
