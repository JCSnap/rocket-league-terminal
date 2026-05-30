use super::{DrawBackend, Renderable};
use crate::math::{Shape, Circle, Point};
use ratatui::{
    DefaultTerminal,
    Frame,
    style::Color,
    text::{Text, Line},
    widgets::{Paragraph, canvas::{Canvas, Circle as RatatuiCircle}},
};

pub struct RatatuiBackend {
    terminal: DefaultTerminal
}

impl RatatuiBackend {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init()
        }
    }
}

fn draw_circle(circle: &Circle, frame: &mut Frame, width: u16, height: u16) {
    let canvas = Canvas::default()
        .x_bounds([0.0, width as f64])
        .y_bounds([0.0, height as f64])
        .paint(|ctx| {
            ctx.draw(&RatatuiCircle {
                x: circle.origin.x as f64 * width as f64,
                y: circle.origin.y as f64 * height as f64,
                radius: circle.radius as f64,
                color: Color::White,
            });
        });
    frame.render_widget(canvas, frame.area());
}

impl DrawBackend for RatatuiBackend {
    fn render_home(&mut self) {
        self.terminal.draw(|frame| {
            let text = Text::from(vec![
                Line::from("ROCKET LEAGUE"),
                Line::from(""),
                Line::from("SPACE - Start"),
                Line::from("ESC   - Quit"),
            ]);
            frame.render_widget(Paragraph::new(text).centered(), frame.area());
        }).unwrap();
    }

    fn render_game(&mut self, renderables: &[&dyn Renderable], width: u16, height: u16) {
        self.terminal.draw(|frame| {
            for renderable in renderables {
                match renderable.shape() {
                    Shape::Circle(circle) => draw_circle(&circle, frame, width, height),
                }
            }
        }).unwrap();
    }
}
