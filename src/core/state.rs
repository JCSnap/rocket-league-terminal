use crate::game::Player;

pub struct GameState {
    player: Player
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn update(&self, delta_time: f32) {
        println!("updating {}", delta_time);
    }
}
