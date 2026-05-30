use super::DrawBackend;
use crate::math::Point;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub struct RatatuiBackend {
}

impl DrawBackend for RatatuiBackend {
    fn draw_circle(&self, origin: Point, radius: f32) {
    }
}
