use crate::assests::*;
use crate::config::*;
use crate::functions::*;
use macroquad::prelude::*;

// food is a data , it doesn't need update, do you update rock ? it depends on snake,
//so it's data
// snake do have update because it independent
pub struct Food<'a> {
    pub pos: Vec<Vec2>,
    pub texture: &'a Assests,
    pub _color: Color,
}
impl<'a> Food<'a> {
    pub fn new(pos: Vec<Vec2>, texture: &'a Assests, _color: Color) -> Self {
        Self {
            pos,
            texture,
            _color,
        }
    }
    pub fn draw(&self) {
        for food_cell in &self.pos {
            // draw_rectangle(food_cell.x, food_cell.y, FOOD_SIZE, FOOD_SIZE, self.color);
            draw_texture(&self.texture.food_sprite, food_cell.x, food_cell.y, WHITE);
        }
    }
    pub fn reset(&mut self) {
        self.pos.clear();
        let num_food = 2;
        for _i in 0..=num_food {
            let food_cell: Vec2 = Vec2::new(random_spot(WIDTH), random_spot(HEIGHT));
            self.pos.push(food_cell);
        }
    }
}
