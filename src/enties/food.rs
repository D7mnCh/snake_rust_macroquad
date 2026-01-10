use super::snake::Snake;
use crate::assests::*;
use crate::config::*;
use macroquad::prelude::*;

pub struct Food<'a> {
    pub pos: Vec2,
    pub texture: &'a Assests,
    pub color: Color,
}
impl<'a> Food<'a> {
    pub fn new(pos: Vec2, texture: &'a Assests, color: Color) -> Self {
        Self {
            pos,
            texture,
            color,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, FOOD_SIZE, FOOD_SIZE, self.color);
        // draw_texture(&self.texture.food_sprite, self.pos.x, self.pos.y, WHITE);
    }
    pub fn update(&mut self, snake: &mut Snake) {
        if snake.pos[0] == self.pos {
            // make new cell for the snake, spwan it on his last  tail cell
            let new_cell = Vec2::new(
                snake.pos[snake.pos.len() - 1].x,
                snake.pos[snake.pos.len() - 1].y,
            );
            snake.pos.push(new_cell);
            // snake.pos.push(value);
            let mut x_pos = rand::gen_range(0, WIDTH);
            let mut y_pos = rand::gen_range(0, HEIGHT);
            // how much do i need to make that range multipliable by 40 ?
            let result_dev_x = x_pos % 40;
            let result_dev_y = y_pos % 40;

            x_pos -= result_dev_x;
            y_pos -= result_dev_y;

            self.pos = Vec2::new(x_pos as f32, y_pos as f32);
        }
    }
}
