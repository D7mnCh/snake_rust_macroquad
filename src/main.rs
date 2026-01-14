mod app;
mod assests;
mod config;
mod enties;
mod functions;
mod ui;

use macroquad::prelude::*;

use crate::app::*;
use crate::assests::*;
use crate::config::*;
use crate::enties::food::Food;
use crate::enties::snake::{direction::Direction, *};
use crate::functions::*;
use crate::ui::*;

#[macroquad::main(window_conf)]
async fn main() {
    // making a seed
    rand::srand(macroquad::miniquad::date::now() as _);

    // for game loop
    let game_running: bool = false;
    let is_app_running: bool = true;
    // making score
    let mut score = 0;

    // loading assets
    let assets = Assests::load().await;

    // building snake
    let mut snake_cells_position = Vec::new();
    for i in 0..=10 {
        let new_cell = Vec2::new(
            WIDTH as f32 / 2.,
            HEIGHT as f32 / 2. + (GRID_BOX * i) as f32,
        );
        snake_cells_position.push(new_cell);
    }
    let snake_head_dir = Direction::Up;
    let snake: Snake = Snake::new(snake_cells_position, snake_head_dir, &assets);

    // building food
    let food_pos = vec![
        Vec2::new(random_spot(WIDTH), random_spot(HEIGHT)),
        Vec2::new(random_spot(WIDTH), random_spot(HEIGHT)),
        Vec2::new(random_spot(WIDTH), random_spot(HEIGHT)),
    ];
    let food = Food::new(
        food_pos,
        &assets,
        Color {
            r: 0.396078,
            g: 0.258824,
            b: 0.043137,
            a: 1.0,
        },
    );

    // building Ui
    let ui: Ui = Ui::new(game_running);

    // building state, run the main loop
    let mut game: App = App::new(snake, food, ui, &mut score, game_running, is_app_running);
    game.run().await;
}

/*
- can you please fix the position of the window on the web...
*/
