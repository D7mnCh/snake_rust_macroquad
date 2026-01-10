use crate::config::*;
use crate::enties::{food::Food, snake::Snake};
// use crate::functions::*;

use macroquad::prelude::*;

pub struct App<'a> {
    snake: Snake<'a>,
    food: Food<'a>,
    score: i32,
    game_running: bool,
}
impl<'a> App<'a> {
    pub fn new(snake: Snake<'a>, food: Food<'a>, score: i32, game_running: bool) -> Self {
        Self {
            snake,
            food,
            score,
            game_running,
        }
    }
    // pub fn config_input_handling (&mut self){}
    pub fn input_handling(&mut self) -> i32 {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            self.game_running = false
        } else if is_key_pressed(KeyCode::Space) {
            //TODO add feature to suspend the game
        }

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
        self.snake.draw();
        self.food.draw();
    }
    pub fn score(&mut self) {
        if self.snake.pos[0] == self.food.pos {
            self.score += 1;
            println!("nice catch friendo");
        }
    }
    pub async fn run(&mut self) {
        let mut time_since_last_update = 0.0;
        let mut input_handling_counter: i32 = 0;

        while self.game_running {
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
            }
            self.draw();
            next_frame().await;
        }
    }
}
