use crate::config::*;
use macroquad::prelude::*;
pub struct Ui;
impl Ui {
    pub fn new() -> Self {
        Self {}
    }
    pub fn display_score(&self, snake_score: &i32, config: &Config) {
        let score = format!("Score: {}", snake_score);
        draw_text(
            score.as_str(),
            config.screen_width as f32 / 2.7,
            config.screen_height as f32 / 17.,
            50.,
            ORANGE,
        );
    }
    pub fn display_pause(&self,config: &Config) {
        draw_text(
            "Press space to play",
            config.screen_width as f32 / 5.,
            config.screen_height as f32 / 1.07,
            55.,
            GRAY,
        );
    }
    pub fn display_defeat(&self, config: &Config) {
        draw_text(
            "Defaited",
            config.screen_width as f32 / 3.4,
            config.screen_height as f32 / 3.5,
            80.,
            GRAY,
        );
    }
    pub fn display_play_again_or_quit(&self, config: &Config) {
        draw_text(
            "Press R to play again",
            config.screen_width as f32 / 5.7,
            config.screen_height as f32 / 1.5,
            60.,
            GRAY,
        );
        draw_text("or", config.screen_width as f32 / 2.2, config.screen_height as f32 / 1.7, 60., GRAY);
        draw_text(
            "Press Q/Escape to quit",
            config.screen_width as f32 / 6.2,
            config.screen_height as f32 / 2.,
            60.,
            GRAY,
        );
    }
    pub fn display_padding(&self, config: &Config) {
        let line_thikness = 5.;
        draw_line(0., 0., config.screen_width as f32, 0., line_thikness, DARKBLUE);
        draw_line(
            0.,
            config.screen_height as f32,
            config.screen_width as f32,
            config.screen_height as f32,
            line_thikness,
            DARKBLUE,
        );
        draw_line(0., 0., 0., config.screen_height as f32, line_thikness, DARKBLUE);
        draw_line(
            config.screen_width as f32,
            0.,
            config.screen_width as f32,
            config.screen_height as f32,
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
            "Move:  a w s d  (Normal)",
            center_x - magic_num,
            100.0,
            text_size,
            color,
        );
        draw_text(
            "Move:  h k j l (Hard)",
            center_x - magic_num,
            150.0,
            text_size,
            color,
        );
        draw_text("Restart:  r", center_x - magic_num, 250.0, text_size, color);
        draw_text(
            "Pause:  space",
            center_x - magic_num,
            300.0,
            text_size,
            color,
        );
        draw_text(
            "Quit:  q / esc",
            center_x - magic_num,
            350.0,
            text_size,
            color,
        );
    }
    pub fn _grid_draw(&self, config: &Config) {
        for x in 0..=config.screen_width {
            for y in 0..=config.screen_width {
                if x % config.grid_box as i32 == 0 && y % config.grid_box as i32 == 0 {
                    draw_line(x as f32, 0., x as f32, config.screen_height as f32, 1., GRAY);
                    draw_line(0., y as f32, config.screen_width as f32, y as f32, 1., GRAY);
                }
            }
        }
    }
}
