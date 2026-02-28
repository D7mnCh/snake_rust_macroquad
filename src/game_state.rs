use macroquad::input::{KeyCode, is_key_pressed};
pub enum GameState {
    Running,
    Resetting,
    Pausing,
    GameOver,
    Quit
}

impl GameState {
    pub fn config_input_handling(&mut self) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            match self {
                _ => *self = GameState::Quit,
            }
        } else if is_key_pressed(KeyCode::Space) {
            match self {
                GameState::Running => *self = GameState::Pausing,
                GameState::Pausing => *self = GameState::Running,
                GameState::Resetting => *self = GameState::Running,
                _ => ()
            }
        } else if is_key_pressed(KeyCode::R) {
            match self {
                _ => *self = GameState::Resetting,
            }
        }
    }
}
