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

    // let _snake_tail: Vec<Vec2> = Vec::new();
    let snake_tail_position = vec![Vec2::new(
        WIDTH as f32 / 2.,
        HEIGHT as f32 / 2. + GRID_BOX as f32,
    )]; // (440, 400)

    let snake_head_position = Vec2::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.); //(400, 400)
    let snake_head_dir = Direction::Up;

    let snake: Snake = Snake::new(
        snake_head_position,
        snake_tail_position,
        snake_head_dir,
        GOLD,
    );

    let mut game: App = App::new(snake, game_running);
    game.run().await;
}
