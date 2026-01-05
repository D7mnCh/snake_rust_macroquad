// use grids first, with the snake move on one direcetion
use crate::config::*;
pub mod direction;

use direction::*;
use macroquad::prelude::*;

pub struct Snake {
    position: Vec2,
    size: Vec2,
    dir: Direction,
    color: Color,
}
impl Snake {
    pub fn new(position: Vec2, size: Vec2, dir: Direction, color: Color) -> Self {
        Self {
            position,
            size,
            dir,
            color,
        }
    }
    pub fn input_handling(&mut self) {
        if is_key_pressed(KeyCode::J) && self.dir.can_change_to(Direction::Down) {
            self.dir = Direction::Down
        } else if is_key_pressed(KeyCode::K) && self.dir.can_change_to(Direction::Up) {
            self.dir = Direction::Up
        } else if is_key_pressed(KeyCode::H) && self.dir.can_change_to(Direction::Left) {
            self.dir = Direction::Left
        } else if is_key_pressed(KeyCode::L) && self.dir.can_change_to(Direction::Right) {
            self.dir = Direction::Right
        }
    }
    fn wall_collistion(&mut self) {
        if self.position.x + self.size.x > WIDTH as f32 {
            self.position.x = 0.
        } else if self.position.x < 0. as f32 {
            self.position.x = WIDTH as f32 - self.size.x
        } else if self.position.y + self.size.y > HEIGHT as f32 {
            self.position.y = 0.
        } else if self.position.y < 0. {
            self.position.y = HEIGHT as f32 - self.size.y
        }
    }
    pub fn update(&mut self) {
        // bug i can change to opposite direction if i pressed top/bottom and then the opposite direction
        //as an exmaple !
        // accept the input but enable changes to the snake only after update logic happend
        // must not update the snake pos between the frames
        match self.dir {
            Direction::Up => self.position.y -= GRID_BOX,
            Direction::Down => self.position.y += GRID_BOX,
            Direction::Right => self.position.x += GRID_BOX,
            Direction::Left => self.position.x -= GRID_BOX,
        }
        self.wall_collistion();
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }
}
