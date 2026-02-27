use crate::config::Config;
use crate::entities::food::Food;
use crate::entities::snake::Snake;
use crate::functions::random_spot;
use macroquad::math::Vec2;

pub fn checking_food_pos(snake: &Snake, food: &mut Food, config: &Config) {
    for snake_cell in &snake.pos {
        for food_cell in &mut food.pos {
            if *food_cell == *snake_cell {
                *food_cell = Vec2::new(
                    random_spot(config.screen_width),
                    random_spot(config.screen_height),
                );
            }
        }
    }
}
// system
pub fn spawn_food(snake: &mut Snake, food: &mut Food) {
    food.pos.iter().for_each(|food_cell| {
        if snake.pos[0] == *food_cell {
            snake.grow();
        }
    });
    // put this bellow the current system when called
    //checking_food_pos(&Snake, &mut Food, &Config);
}
pub fn add_score(snake: &mut Snake, food: &Food) {
    food.pos.iter().for_each(|food_cell| {
        if snake.pos[0] == *food_cell {
            snake.score += 1;
        }
    });
}
