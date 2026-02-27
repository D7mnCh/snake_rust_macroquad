pub mod food;
pub mod snake;
use crate::assets::Assets;
use crate::entities::food::Food;
use crate::entities::snake::Snake;
use crate::config::Config;
use crate::traits::*;
pub struct Entities {
    pub snake: Snake,
    pub food: Food,
}
// ok so port sysetms here
impl Entities {
    pub fn new(config: &Config) -> Self {
        let snake: Snake = Snake::new(config);
        let food = Food::new(config);
        Self { snake, food }
    }
    pub fn draw (&self, assets: &Assets) {
        self.snake.draw(assets);
        self.food.draw(assets);
    }
}
