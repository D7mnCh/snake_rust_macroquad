use crate::assets::{
    Assets,
    SpriteId::{self, *},
};
use crate::config::Config;
use crate::functions::*;
use crate::traits::*;
use macroquad::prelude::*;

pub struct Food {
    pub pos: Vec<Vec2>,
    pub size: f32,
    pub sprite: SpriteId,
}
impl Renderable for Food {
    async fn draw(&self, assets: &Assets) {
        let food_texture = self.sprite.get_texture(assets).await;
        for food_cell in &self.pos {
            draw_texture(food_texture, food_cell.x, food_cell.y, WHITE);
        }
    }
}

// for update method, you can make collision detection and return something, and then pass as argument to the current update method that the Food struct have (clean)
impl Food {
    pub fn new(config: &Config) -> Self {
        let mut pos = Vec::new();
        let num_of_spawned_food = 2;
        for _ in 0..=num_of_spawned_food {
            let food_pos = Vec2::new(random_spot(config.screen_width), random_spot(config.screen_height));
            pos.push(food_pos);
        }
        let sprite = FoodSprite;
        let size = config.grid_box;

        Self {
            pos,
            size,
            sprite,
        }
    }
    pub fn reset(&mut self, config: &Config) {
        self.pos.clear();
        let num_food = 2;
        for _ in 0..=num_food {
            let food_cell: Vec2 = Vec2::new(random_spot(config.screen_width), random_spot(config.screen_height));
            self.pos.push(food_cell);
        }
    }
}
