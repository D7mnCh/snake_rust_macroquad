use crate::config::Config;
use crate::entities::food::Food;
use crate::entities::snake::Snake;
use crate::functions::random_spot;
use macroquad::math::Vec2;
/*
// with those two functions i want to update food on the food mod

pub fn collision_detection_food_snake_head (snake: &mut Snake, food: &mut Food, config: &Config) -> bool {
    for _ in 0..snake.pos.iter().len(){
        for food_cell in &mut food.pos {
            if *food_cell == snake.pos[0] {
                return true;
            }
        }
    }
    false
}

pub fn collision_detection_food_snake_tail (snake: &mut Snake, food: &mut Food, config: &Config)-> (bool,usize) {
    for i in 0..snake.pos.iter().len(){
        for food_cell in &mut food.pos {
            if *food_cell == snake.pos[i] && i > 0{
                return (true,i);
            }
        }
    }
    (false,0)
}
*/

pub fn spawn_food(snake: &mut Snake, food: &mut Food, config: &Config){
    for i in 0..snake.pos.iter().len(){
        for food_cell in &mut food.pos {
            if *food_cell == snake.pos[0] {
                *food_cell = Vec2::new(
                    random_spot(config.screen_width),
                    random_spot(config.screen_height),
                );
                    snake.grow();
            }
            if *food_cell == snake.pos[i] && i > 0{
                *food_cell = Vec2::new(
                    random_spot(config.screen_width),
                    random_spot(config.screen_height),
                );
            }
        }
    }
}

pub fn add_score(snake: &mut Snake, food: &Food) {
    food.pos.iter().for_each(|food_cell| {
        if snake.pos[0] == *food_cell {
            snake.score += 1;
        }
    });
}
