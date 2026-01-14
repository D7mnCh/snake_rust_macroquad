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
    pub fn display_pause(&self) {
        draw_text(
            "Press space to play",
            WIDTH as f32 / 5.,
            HEIGHT as f32 / 1.07,
            55.,
            GRAY,
        );
    }
    pub fn dispay_defait(&mut self) {
        draw_text(
            "Defaited",
            WIDTH as f32 / 3.4,
            HEIGHT as f32 / 3.5,
            80.,
            GRAY,
        );
    }
    pub fn display_play_again_or_quit(&mut self) {
        draw_text(
            "Press R to play again",
            WIDTH as f32 / 5.7,
            HEIGHT as f32 / 1.5,
            60.,
            GRAY,
        );
        draw_text("or", WIDTH as f32 / 2.2, HEIGHT as f32 / 1.7, 60., GRAY);
        draw_text(
            "Press Q/Escape to quit",
            WIDTH as f32 / 6.2,
            HEIGHT as f32 / 2.,
            60.,
            GRAY,
        );
    }
}
