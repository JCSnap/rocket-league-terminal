use crate::game::Player;

pub struct GameState {
    pub player: Player
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn update(&self, delta_time: f32) {
    }
}
