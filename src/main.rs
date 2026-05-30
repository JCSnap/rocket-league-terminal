mod core;
mod math;
mod game;
mod constant;

use crate::core::Engine;

fn main() {
    let mut engine = Engine::new();

    engine.run();
}
