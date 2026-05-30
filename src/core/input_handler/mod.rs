mod keyboard_input_handler;
pub use keyboard_input_handler::KeyboardInputHandler;

pub enum Action {
    GoHome,
    StartGame,
    Quit,
    Up,
    Down,
    Left,
    Right,
    None
}

pub trait InputHandler {
    fn poll(&mut self) -> Vec<Action>;
}
