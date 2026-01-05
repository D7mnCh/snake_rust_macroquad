mod app;
mod config;
mod enties;
mod systems;

use macroquad::prelude::*;

use crate::app::*;
use crate::config::*;
use crate::enties::snake::{direction::Direction, *};
use crate::systems::*;

#[macroquad::main(window_conf)]
async fn main() {
    let game_running: bool = true;

    let snake_pos = Vec2::new(WIDTH as f32 / 2., HEIGHT as f32 / 2.);
    let snake_size = Vec2::new(SNAKE_SIZE, SNAKE_SIZE);
    let snake_dir = Direction::Up;
    let snake: Snake = Snake::new(snake_pos, snake_size, snake_dir, GOLD);

    let mut game: App = App::new(snake, game_running);
    game.run().await;
}
