pub mod food;
pub mod snake;
use crate::assets::Assets;
use crate::config::Config;
use crate::entities::food::Food;
use crate::entities::snake::Snake;
use crate::traits::*;

pub struct Entities {
    pub snake: Snake,
    pub food: Food,
}

impl Entities {
    pub fn new(config: &Config) -> Self {
        let snake: Snake = Snake::new(config);
        let food = Food::new(config);
        Self { snake, food }
    }

    pub async fn draw(&self, assets: &Assets) {
        self.snake.draw(assets).await;
        self.food.draw(assets).await;
    }
}
