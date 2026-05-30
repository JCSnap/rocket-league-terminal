use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use super::{Action, InputHandler};
use std::time::Duration;
use std::collections::HashSet;

pub struct KeyboardInputHandler {
    held_keys: HashSet<KeyCode>
}

impl KeyboardInputHandler {
    pub fn new() -> Self {
        Self {
            held_keys: HashSet::new()
        }
    }
}


impl InputHandler for KeyboardInputHandler {
    fn poll(&mut self) -> Vec<Action> {
        let mut actions = vec![];

        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.kind {
                    KeyEventKind::Press => {
                        self.held_keys.insert(key.code);
                        match key.code {
                            KeyCode::Char(' ') => actions.push(Action::StartGame),
                            KeyCode::Esc => actions.push(Action::GoHome),
                            KeyCode::Char('q') => actions.push(Action::Quit),
                            _ => {}
                        }
                    }
                    KeyEventKind::Release => { self.held_keys.remove(&key.code); }
                    _ => {}
                }
            }
        }

        let movement_key_map = [
            (KeyCode::Char('w'), Action::Up),
            (KeyCode::Char('a'), Action::Left),
            (KeyCode::Char('s'), Action::Down),
            (KeyCode::Char('d'), Action::Right),
        ];

        for (key, action) in movement_key_map {
            if self.held_keys.contains(&key) { actions.push(action); }
        }

        actions
    }
}
