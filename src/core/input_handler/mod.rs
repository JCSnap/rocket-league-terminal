mod keyboard_input_handler;
pub use keyboard_input_handler::KeyboardInputHandler;

pub enum Action {
    GoHome,
    StartGame,
    Quit,
    None
}

pub trait InputHandler {
    fn poll(&self) -> Action;
}
