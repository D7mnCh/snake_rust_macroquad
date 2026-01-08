use crate::config::*;
use crate::enties::snake::Snake;
use crate::systems::*;

use macroquad::prelude::*;

pub struct App {
    //snake_body
    snake: Snake,
    game_running: bool,
}
impl App {
    pub fn new(snake: Snake, game_running: bool) -> Self {
        Self {
            snake,
            game_running,
        }
    }
    // pub fn config_input_handling (&mut self){}
    pub fn input_handling(&mut self) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            self.game_running = false
        } else if is_key_pressed(KeyCode::Space) {
            //TODO add feature to suspend the game
        }

        self.snake.input_handling();
    }
    pub fn update(&mut self) {
        self.snake.update();
    }
    pub fn draw(&mut self) {
        clear_background(BLACK);
        grid_draw();
        self.snake.draw();
    }
    pub async fn run(&mut self) {
        let mut time_since_last_update = 0.0;

        while self.game_running {
            let dt = get_frame_time();
            //with this i am combining frames
            time_since_last_update += dt;

            self.input_handling();
            if time_since_last_update >= TARGET_FPS {
                self.update();
                time_since_last_update = 0.0;

                // Logging
                // println!("[Info] head direction : {:?}", self.snake.head_dir);
                println!();
            }
            self.draw();
            next_frame().await;
        }
    }
}
