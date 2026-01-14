use crate::config::*;
use crate::enties::{food::Food, snake::Snake};
use crate::functions::*;
use crate::ui::*;

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
    // if food spawns inside snake body
    pub fn checking_food_pos(&mut self) {
        for snake_cell in &self.snake.pos {
            for food_cell in &mut self.food.pos {
                if *food_cell == *snake_cell {
                    *food_cell = Vec2::new(random_spot(WIDTH), random_spot(HEIGHT));
                }
            }
        }
    }
    pub fn spawn_food(&mut self) {
        for food_cell in &mut self.food.pos {
            if self.snake.pos[0] == *food_cell {
                self.snake.grow();
            }
        }
        self.checking_food_pos();
    }
    pub fn check_collistion_to_reset(&mut self) -> bool {
        if self.snake.collision() {
            self.game_running = false;

            return true;
        }
        return false;
    }
    pub fn score(&mut self) {
        for food_cell in &self.food.pos {
            if self.snake.pos[0] == *food_cell {
                *self.score += 1;
            }
        }
    }
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
        } else if is_key_pressed(KeyCode::Space) && !self.snake.collision() {
            self.game_running = !self.game_running;
        } else if is_key_pressed(KeyCode::R) {
            self.game_running = false;
            if self.snake.pos.len() != 3 && *self.score != 0 {
                println!("is this even working ?");
                self.food.reset();
                self.checking_food_pos();
            }
            self.snake.reset();

            *self.score = 0;
        }
    }
    pub fn input_handling(&mut self) -> i32 {
        self.snake.input_handling()
    }
    pub fn update(&mut self) {
        self.snake.update();
        self.score();
        self.spawn_food();
        self.check_collistion_to_reset();
    }
    pub fn draw(&mut self) {
        clear_background(BLACK);
        self.snake.draw();
        self.food.draw();
        self.ui.display_padding();
        // self.ui.grid_draw();
        if !self.game_running && !self.snake.collision() {
            self.ui.display_pause();
            self.ui.display_greetings();
        }
        self.ui.display_score(&self.score);

        if self.snake.collision() {
            self.ui.dispay_defait();
            self.ui.display_play_again_or_quit();
        }
    }
    pub async fn run(&mut self) {
        self.checking_food_pos();
        let mut time_since_last_update = 0.0;
        let mut input_handling_counter: i32 = 0;

        while self.is_app_running {
            self.config_input_handling();

            if self.game_running {
                let dt = get_frame_time();
                //with this i am combining frames
                time_since_last_update += dt;

                // sometimes the tick frame (where the actual the update happend) will not run
                //on everyframe
                //so input_handling can run change direction multiple times
                // i want to accept direction modification once, until the tick happend !
                if input_handling_counter == 0 {
                    input_handling_counter = self.input_handling();
                }

                if time_since_last_update >= TARGET_FPS {
                    self.update();
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
