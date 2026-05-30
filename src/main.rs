mod core;
mod math;
mod game;

use crate::core::Engine;
use crate::core::GameState;

fn main() {
    let mut engine = Engine::new();
    let mut game_state: GameState = GameState::new();

    engine.run();

    while engine.is_game_running() {
        let dt = engine.delta_time();
        engine.check_user_input();
        game_state.update(dt);
        engine.render();
    }
}
