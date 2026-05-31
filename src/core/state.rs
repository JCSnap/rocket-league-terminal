use crate::game::{Player, Wall};
use crate::core::{PhysicsBody, Renderable};
use crate::math::Point;

pub struct GameState {
    pub player: Player,
    pub walls: Vec<Wall>
}

impl GameState {
    pub fn new() -> Self {
        let north_wall = Wall::new(Point { x: 0.5, y: 1.0 }, 1.0, 0.01);
        let south_wall = Wall::new(Point { x: 0.5, y: 0.0 }, 1.0, 0.01);
        let east_wall = Wall::new(Point { x: 1.0, y: 0.5 }, 0.01, 1.0);
        let west_wall = Wall::new(Point { x: 0.0, y: 0.5 }, 0.01, 1.0);

        Self {
            player: Player::new(),
            walls: vec![north_wall, south_wall, east_wall, west_wall]
        }
    }

    pub fn update(&self, delta_time: f32) {
    }

    pub fn get_renderables(&mut self) -> Vec<&dyn Renderable> {
        let mut renderables: Vec<&dyn Renderable> = vec![&mut self.player];
        renderables.extend(self.walls.iter().map(|w| w as &dyn Renderable));
        renderables
    }

    pub fn get_physics_bodies(&mut self) -> Vec<&mut PhysicsBody> {
        vec![
            &mut self.player.body
        ]
    }
}
