mod app;
mod config;
mod enties;
mod functions;

use macroquad::prelude::*;

use crate::app::*;
use crate::config::*;
use crate::enties::snake::{direction::Direction, *};
use crate::functions::*;

#[macroquad::main(window_conf)]
async fn main() {
    let game_running: bool = true;

    let mut snake_position = Vec::new();
    for i in 1..10 {
        let new_cell = Vec2::new(
            WIDTH as f32 / 2.,
            HEIGHT as f32 / 2. + (GRID_BOX * i as f32) as f32,
        );
        snake_position.push(new_cell);
    }

    let snake_head_dir = Direction::Up;

    let snake: Snake = Snake::new(snake_position, snake_head_dir, GOLD);

    let mut game: App = App::new(snake, game_running);
    game.run().await;
}
