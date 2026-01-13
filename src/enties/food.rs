use super::snake::Snake;
use crate::assests::*;
use crate::config::*;
use crate::functions::random_spot;
use macroquad::prelude::*;

// food is a data , it doesn't need update, do you update rock ? it depends on snake,
//so it's data
// snake do have update because it independent
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
            // draw_rectangle(food_cell.x, food_cell.y, FOOD_SIZE, FOOD_SIZE, self.color);
            draw_texture(&self.texture.food_sprite, food_cell.x, food_cell.y, WHITE);
        }
    }
    // if you notice here snake struct must not be in food stuct, fix this by removing
    //snake sturct here !!!!!!!!!!
    pub fn update(&mut self, snake: &mut Snake) {
        for food_cell in &mut self.pos {
            if snake.pos[0] == *food_cell {
                // make new cell for the snake, spwan it on his last tail cell

                // update snake
                let new_snake_cell = Vec2::new(
                    snake.pos[snake.pos.len() - 1].x,
                    snake.pos[snake.pos.len() - 1].y,
                );
                *food_cell = Vec2::new(random_spot(WIDTH), random_spot(HEIGHT));

                snake.pos.push(new_snake_cell)
                // update snake
                // spawn only if the cell_snake not as same as that rand spot !
            }
        }
    }
}
