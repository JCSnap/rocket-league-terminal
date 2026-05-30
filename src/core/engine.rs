use crate::core::{GameState, Renderer, Renderable};

pub enum GameScreen {
    Home,
    Playing
}

pub struct Engine {
    screen: GameScreen,
    fps: u32,
    game_state: GameState,
    renderer: Renderer
}

impl Engine {
    pub fn new() -> Self {
        Self {
            screen: GameScreen::Home,
            fps: 60,
            game_state: GameState::new(),
            renderer: Renderer::new()
        }
    }

    pub fn run(&mut self) {
        loop {
            self.check_user_input();
            match self.screen {
                GameScreen::Home => self.render_home(),
                GameScreen::Playing => self.update()
            }
        }
    }

    pub fn update(&mut self) {
        let dt = self.delta_time();
        self.check_user_input();
        self.game_state.update(dt);
        self.render_game();
    }

    pub fn delta_time(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn check_user_input(&self) {
    }

    pub fn render_home(&mut self) {
        self.renderer.render_home();
    }

    pub fn render_game(&mut self) {
        let renderables: Vec<&dyn Renderable> = vec![
            &self.game_state.player
        ];
        self.renderer.render_game(&renderables);
    }

}
