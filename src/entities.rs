pub mod food;
pub mod snake;
use crate::entities::snake::Snake;
use crate::entities::food::Food;
pub struct Entities {
    snake: Snake,
    food: Food,
}
impl Entities {
    pub fn new () -> Self {
        let snake: Snake = Snake::new();
        let food = Food::new();
        Self {
            snake,
            food
        }
    }
}
