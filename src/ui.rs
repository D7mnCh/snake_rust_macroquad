use crate::config::*;
use macroquad::prelude::*;
pub struct Ui {}
impl Ui {
    pub fn new() -> Self {
        Self {}
    }
    pub fn display_score(&self, score: &i32) {
        let score = format!("Score: {}", score);
        draw_text(
            score.as_str(),
            WIDTH as f32 / 2.7,
            HEIGHT as f32 / 17.,
            50.,
            ORANGE,
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
    pub fn dispay_defait(&self) {
        draw_text(
            "Defaited",
            WIDTH as f32 / 3.4,
            HEIGHT as f32 / 3.5,
            80.,
            GRAY,
        );
    }
    pub fn display_play_again_or_quit(&self) {
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
    pub fn display_padding(&self) {
        let line_thikness = 5.;
        draw_line(0., 0., WIDTH as f32, 0., line_thikness, DARKBLUE);
        draw_line(
            0.,
            HEIGHT as f32,
            WIDTH as f32,
            HEIGHT as f32,
            line_thikness,
            DARKBLUE,
        );
        draw_line(0., 0., 0., HEIGHT as f32, line_thikness, DARKBLUE);
        draw_line(
            WIDTH as f32,
            0.,
            WIDTH as f32,
            HEIGHT as f32,
            line_thikness,
            DARKBLUE,
        );
    }
    pub fn display_greetings(&self) {
        let text_size = 55.0;
        let center_x = 800.0 / 2.0;
        let magic_num = 200.;
        let color = GRAY;

        draw_text(
            "Move:  h j k l",
            center_x - magic_num,
            100.0,
            text_size,
            color,
        );
        draw_text("Restart:  r", center_x - magic_num, 150.0, text_size, color);
        draw_text(
            "Pause:  space",
            center_x - magic_num,
            200.0,
            text_size,
            color,
        );
        draw_text(
            "Quit:  q / esc",
            center_x - magic_num,
            250.0,
            text_size,
            color,
        );
    }
    pub fn _grid_draw(&self) {
        for x in 0..=WIDTH {
            for y in 0..=WIDTH {
                if x % GRID_BOX as i32 == 0 && y % GRID_BOX as i32 == 0 {
                    draw_line(x as f32, 0., x as f32, HEIGHT as f32, 1., GRAY);
                    draw_line(0., y as f32, WIDTH as f32, y as f32, 1., GRAY);
                }
            }
        }
    }
}
