mod app;
mod assests;
mod config;
mod enties;
mod functions;

use macroquad::prelude::*;

use crate::app::*;
use crate::assests::*;
use crate::config::*;
use crate::enties::food::Food;
use crate::enties::snake::{direction::Direction, *};
use crate::functions::*;

#[macroquad::main(window_conf)]
async fn main() {
    let game_running: bool = true;
    rand::srand(macroquad::miniquad::date::now() as _);

    let assets = Assests::load().await;

    let mut snake_cells_position = Vec::new();
    for i in 0..=3 {
        let new_cell = Vec2::new(
            WIDTH as f32 / 2.,
            HEIGHT as f32 / 2. + (GRID_BOX * i as f32) as f32,
        );
        snake_cells_position.push(new_cell);
    }
    let snake_head_dir = Direction::Up;
    let snake: Snake = Snake::new(snake_cells_position, snake_head_dir, &assets);

    let food_pos = Vec2::new(480., 520.);
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

    let score = 0;

    let mut game: App = App::new(snake, food, score, game_running);
    game.run().await;
}
