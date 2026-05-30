use crossterm::event::{self, Event, KeyCode};
use super::{Action, InputHandler};
use std::time::Duration;

pub struct KeyboardInputHandler {
}

impl KeyboardInputHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl InputHandler for KeyboardInputHandler {
    fn poll(&self) -> Action {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                return match key.code {
                    KeyCode::Char(' ') => Action::StartGame,
                    KeyCode::Esc => Action::GoHome,
                    KeyCode::Char('q') => Action::Quit,
                    _ => Action::None,
                };
            }
        }
        Action::None
    }
}
