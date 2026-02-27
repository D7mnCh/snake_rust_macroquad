use macroquad::input::{is_key_pressed, KeyCode};
pub enum GameState {
    Running,
    Resetting,
    Pausing,
    Defeat,
    GameOver,
}
impl GameState {
    pub fn config_input_handling(&mut self) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            match self {
                _ => *self = GameState::GameOver
            }
        } else if is_key_pressed(KeyCode::Space) /* && !self.snake.collision() */{
            match self {
                _ => *self = GameState::Pausing
            }
        } else if is_key_pressed(KeyCode::R) {
            match self {
                _ => *self = GameState::Resetting
            }
            // this logic gonna be inside the main loop for now
            /*
            if self.snake.pos.len() != 3 && *self.score != 0 {
                println!("is this even working ?");
                self.food.reset();
                self.checking_food_pos();
            }
            self.snake.reset();

            *self.score = 0;
            */
        }
    }
}
