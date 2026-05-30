use crate::core::{GameState, Renderer, Renderable};

pub struct Engine {
    is_running: bool,
    fps: u32,
    game_state: GameState,
    renderer: Renderer
}

impl Engine {
    pub fn new() -> Self {
        Self {
            is_running: false,
            fps: 60,
            game_state: GameState::new(),
            renderer: Renderer::new()
        }
    }

    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            let dt = self.delta_time();
            self.check_user_input();
            self.game_state.update(dt);
        }
    }

    pub fn delta_time(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn check_user_input(&self) {
        println!("checking user input")
    }

    pub fn render(&self) {
        let renderables: Vec<&dyn Renderable> = vec![
            &self.game_state.player
        ];
        self.renderer.render(&renderables);
    }

}
