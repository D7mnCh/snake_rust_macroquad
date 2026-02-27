use crate::assets::Assets;
use crate::systems::*;
use crate::config::Config;
use crate::entities::Entities;
use crate::game_state::GameState;
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
        //rand::srand(macroquad::miniquad::date::now() as _);
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
        spawn_food(&mut self.entities.snake, &mut self.entities.food);
        //check_collistion_to_reset();
    }
    pub fn draw(&mut self) {
        clear_background(BLACK);
        self.entities.draw(&self.assets);
        self.ui.display_padding(&self.config);
        // self.ui.grid_draw();
        /*
        if !self.game_running && !self.entities.snake.collision() {
            self.ui.display_pause();
            self.ui.display_greetings();
        }
        self.ui.display_score(&self.score);

        if self.entities.snake.collision() {
            self.ui.dispay_defait();
            self.ui.display_play_again_or_quit();
        }
        */
    }
    pub async fn run(&mut self) {
        checking_food_pos(&self.entities.snake, &mut self.entities.food, &self.config);
        let mut time_since_last_update = 0.0;

        loop {
            self.game_state.config_input_handling();
            match self.game_state {
                GameState::Running =>{
                    let dt = get_frame_time();
                    //with this i am combining frames
                    time_since_last_update += dt;

                    // sometimes the tick frame (where the actual the update happend) will not run
                    //on everyframe
                    //so input_handling can run change direction multiple times
                    // i want to accept direction modification once, until the tick happend !
                    if time_since_last_update >= self.config.target_fps {
                        self.entities.snake.input_handling();
                        self.update();
                        self.ui.display_score(&self.entities.snake.score, &self.config);

                        time_since_last_update = 0.0;
                        // Logging
                        for (i, cell) in self.entities.snake.pos.iter().enumerate() {
                            println!("[Info] cell num {i} position is : {:?}", cell);
                        }
                        println!();
                        println!("[Info] entities.snake score: {}", self.entities.snake.score);
                    }
                },
                GameState::Resetting => {
                    todo!()
                },
                GameState::Pausing => {
                    self.ui.display_pause(&self.config);
                    self.ui.display_greetings();
                },
                GameState::GameOver => {
                    todo!()
                },
                GameState::Defeat => {
                    self.ui.display_defeat(&self.config);
                    self.ui.display_play_again_or_quit(&self.config);
                }
            }


            self.draw();
            next_frame().await;
        }
    }
}
