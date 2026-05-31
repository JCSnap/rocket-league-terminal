use crate::math::Vec2;

// world
pub const GRAVITY: Vec2 = Vec2 { x: 0.0, y: -1.0 };
pub const DRAG_COEFFICIENT: f32 = 10.0;

// player
pub const PLAYER_MASS: f32 = 10.0;
pub const PLAYER_RADIUS: f32 = 5.0;
pub const PLAYER_STARTING_X: f32 = 0.1;
pub const PLAYER_STARTING_Y: f32 = 0.1;
pub const PLAYER_INPUT_FORCE_MAG: f32 = 1.5;

pub const PLAYER_MAX_INPUT_FORCE_COMPONENT_MAG: f32 = 3.0;
