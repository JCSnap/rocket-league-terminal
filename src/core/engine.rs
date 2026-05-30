pub struct Engine {
    is_running: bool,
    fps: u32,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            is_running: false,
            fps: 60
        }
    }

    pub fn run(mut &self) {
        self.is_running = true;
    }

    pub fn is_game_running(&self) -> bool {
        self.is_running
    }

    pub fn delta_time(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn check_user_input(&self) {
        println!("checking user input")
    }

    pub fn render(&self) {
        println!("rendering")
    }
}
