use crate::assets::Assets;
use crate::config::Config;
use crate::entities::Entities;
use crate::game_state::GameState;
use crate::systems::*;
use crate::ui::Ui;
use macroquad::prelude::*;

pub struct App {
    entities: Entities,
    config: Config,
    game_state: GameState,
    assets: Assets,
    ui: Ui,
}
impl App {
    pub async fn new() -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);

        let config = Config::new();
        let assets = Assets::load().await;
        let entities = Entities::new(&config);
        let ui: Ui = Ui::new();
        let game_state = GameState::Pausing;

        Self {
            config,
            ui,
            assets,
            entities,
            game_state,
        }
    }
    pub fn update(&mut self) {
        self.entities.snake.update(&self.config);
        add_score(&mut self.entities.snake, &mut self.entities.food);
        spawn_food(&mut self.entities.snake, &mut self.entities.food, &self.config);
    }
    pub async fn draw(&mut self) {
        // i should clear frames but it just works i guess
        //clear_background(BLACK);
        self.entities.draw(&self.assets).await;
        self.ui.display_padding(&self.config);
        self.ui._grid_draw(&self.config);
    }
    pub async fn run(&mut self) {
        let mut time_since_last_update = 0.0;
        // to update snake only one time before the tick happen
        let mut snake_input_direction_counting = 0;

        loop {
            self.draw().await;
            self.game_state.config_input_handling();

            match self.game_state {
                GameState::Running => {
                    self.entities.snake.input_handling(&mut snake_input_direction_counting);
                    self.ui
                        .display_score(&self.entities.snake.score, &self.config);

                    let dt = get_frame_time();
                    time_since_last_update += dt;

                    // the tick frame
                    if time_since_last_update >= self.config.target_fps {
                        self.update();
                        if self.entities.snake.check_collision_detection() {
                            self.game_state = GameState::GameOver
                        }

                        time_since_last_update = 0.0;
                        snake_input_direction_counting = 0;
                    }
                        // Logging
                       // for (i, cell) in self.entities.snake.pos.iter().enumerate() {
                       //     println!("[Info] cell num {i} position is : {:?}", cell);
                       // }
                       // println!();
                       // println!("[Info] entities.snake score: {}", self.entities.snake.score);
                }
                GameState::Resetting => {
                    self.entities.snake.reset(&self.config);
                    // TODO if statement doesn't work
                    if self.entities.snake.pos.len() > 3 && self.entities.snake.score > 0 {
                        self.entities.food.reset(&self.config);
                    }
                    self.game_state = GameState::Pausing;
                }
                GameState::Pausing => {
                    self.ui.display_pause(&self.config);
                    self.ui.display_greetings(&self.config);
                }
                GameState::GameOver => {
                    self.ui.display_defeat(&self.config);
                    self.ui.display_play_again_or_quit(&self.config);
                }
                GameState::Quit => break
            }

            next_frame().await;
        }
    }
}
