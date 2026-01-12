use crate::config::*;
use crate::enties::{food::Food, snake::Snake};
use crate::ui::*;
// use crate::functions::*;

use macroquad::prelude::*;

pub struct App<'a> {
    snake: Snake<'a>,
    food: Food<'a>,
    ui: Ui,
    score: &'a mut i32,
    game_running: bool,
    is_app_running: bool,
}
impl<'a> App<'a> {
    pub fn new(
        snake: Snake<'a>,
        food: Food<'a>,
        ui: Ui,
        score: &'a mut i32,
        game_running: bool,
        is_app_running: bool,
    ) -> Self {
        Self {
            snake,
            food,
            ui,
            score,
            game_running,
            is_app_running,
        }
    }
    pub fn config_input_handling(&mut self) {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            self.is_app_running = !self.is_app_running;
        } else if is_key_pressed(KeyCode::Space) {
            self.game_running = !self.game_running;
        }
    }
    pub fn input_handling(&mut self) -> i32 {
        self.snake.input_handling()
    }
    pub fn update(&mut self) {
        self.snake.update();
        self.score();
        self.food.update(&mut self.snake);
    }
    pub fn draw(&mut self) {
        clear_background(BLACK);
        // grid_draw();
        self.ui.display_score(&self.score);
        self.snake.draw();
        self.food.draw();
        if !self.game_running {
            self.ui.display_game_stops();
        }
    }
    pub fn score(&mut self) {
        for food_cell in &self.food.pos {
            if self.snake.pos[0] == *food_cell {
                *self.score += 1;
                println!("nice catch friendo");
            }
        }
    }
    pub async fn run(&mut self) {
        let mut time_since_last_update = 0.0;
        let mut input_handling_counter: i32 = 0;

        while self.is_app_running {
            self.config_input_handling();

            if self.game_running {
                let dt = get_frame_time();
                //with this i am combining frames
                time_since_last_update += dt;

                // sometimes the tick frame (where the update happend) will not run on everyframe
                //so input_handling can run change direction multiple times
                // i want to accept direction modification once, until the tick happend !
                if input_handling_counter == 0 {
                    input_handling_counter = self.input_handling();
                }

                if time_since_last_update >= TARGET_FPS {
                    self.update();
                    self.snake.collision();
                    time_since_last_update = 0.0;
                    input_handling_counter = 0;

                    // Logging
                    for (i, cell) in self.snake.pos.iter().enumerate() {
                        println!("[Info] cell num {i} position is : {:?}", cell);
                    }
                    println!();
                    println!("[Info] snake score: {}", self.score);
                    println!();
                    println!("[Info] the main loop is : {}", self.is_app_running);
                }
            }

            self.draw();
            next_frame().await;
        }
    }
}
