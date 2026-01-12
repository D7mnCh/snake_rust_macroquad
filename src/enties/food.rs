use super::snake::Snake;
use crate::assests::*;
use crate::config::*;
use macroquad::prelude::*;

pub struct Food<'a> {
    pub pos: Vec<Vec2>,
    pub texture: &'a Assests,
    pub color: Color,
}
impl<'a> Food<'a> {
    pub fn new(pos: Vec<Vec2>, texture: &'a Assests, color: Color) -> Self {
        Self {
            pos,
            texture,
            color,
        }
    }
    pub fn draw(&self) {
        for food_cell in &self.pos {
            draw_rectangle(food_cell.x, food_cell.y, FOOD_SIZE, FOOD_SIZE, self.color);
            // draw_texture(&self.texture.food_sprite, self.pos.x, self.pos.y, WHITE);
        }
    }
    pub fn update(&mut self, snake: &mut Snake) {
        for food_cell in &mut self.pos {
            if snake.pos[0] == *food_cell {
                // make new cell for the snake, spwan it on his last tail cell
                let new_food_cell = Vec2::new(
                    snake.pos[snake.pos.len() - 1].x,
                    snake.pos[snake.pos.len() - 1].y,
                );
                snake.pos.push(new_food_cell);

                // spawn new food cells
                let mut x_pos = rand::gen_range(0, WIDTH);
                let mut y_pos = rand::gen_range(0, HEIGHT);
                // how much do i need to make that range multipliable by 40 ?
                let result_dev_x = x_pos % 40;
                let result_dev_y = y_pos % 40;

                x_pos -= result_dev_x;
                y_pos -= result_dev_y;

                *food_cell = Vec2::new(x_pos as f32, y_pos as f32);
            }
        }
    }
}
