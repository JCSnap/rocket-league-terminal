use crate::game::Player;
use crate::core::{PhysicsBody, Renderable};

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

    pub fn get_renderables(&mut self) -> Vec<&dyn Renderable> {
        vec![
            &mut self.player
        ]
    }

    pub fn get_physics_bodies(&mut self) -> Vec<&mut PhysicsBody> {
        vec![
            &mut self.player.body
        ]
    }
}
