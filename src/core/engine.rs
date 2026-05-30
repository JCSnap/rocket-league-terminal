use crate::core::{GameState, Renderer, Renderable, Action, InputHandler, KeyboardInputHandler};

pub enum GameScreen {
    Home,
    Playing
}

pub struct Engine {
    is_running: bool,
    screen: GameScreen,
    fps: u32,
    game_state: GameState,
    renderer: Renderer,
    input_handler: Box<dyn InputHandler>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            is_running: true,
            screen: GameScreen::Home,
            fps: 60,
            game_state: GameState::new(),
            renderer: Renderer::new(),
            input_handler: Box::new(KeyboardInputHandler::new())
        }
    }

    pub fn run(&mut self) {
        while self.is_running {
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

    pub fn check_user_input(&mut self) {
        match self.input_handler.poll() {
            Action::GoHome => { self.screen = GameScreen::Home },
            Action::StartGame => { self.screen = GameScreen::Playing },
            Action::Quit => { self.is_running = false },
            Action::None => {}
        }
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
