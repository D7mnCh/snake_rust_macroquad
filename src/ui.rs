use crate::config::*;
use macroquad::prelude::*;
pub struct Ui {
    game_running: bool,
}
impl Ui {
    pub fn new(game_running: bool) -> Self {
        Self { game_running }
    }
    pub fn display_score(&self, score: &i32) {
        let score = format!("Score: {}", score);
        draw_text(
            score.as_str(),
            WIDTH as f32 / 2.7,
            HEIGHT as f32 / 17.,
            50.,
            GRAY,
        );
    }
    pub fn display_game_stops(&self) {
        draw_text(
            "You Stoped the game !",
            WIDTH as f32 / 4.,
            HEIGHT as f32 / 1.1,
            50.,
            GRAY,
        );
    }
}
