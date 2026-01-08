// use grids first, with the snake move on one direcetion
use crate::config::*;
pub mod direction;

use direction::*;
use macroquad::prelude::*;

pub struct Snake {
    head_pos: Vec2,
    tail_pos: Vec2,
    pub head_dir: Direction,
    color: Color,
}
impl Snake {
    pub fn new(head_pos: Vec2, tail_pos: Vec2, head_dir: Direction, color: Color) -> Self {
        Self {
            head_pos,
            tail_pos,
            head_dir,
            color,
        }
    }
    fn wall_collistion(&mut self) {
        // Head
        if self.head_pos.x + SNAKE_SIZE > WIDTH as f32 {
            self.head_pos.x = 0.
        } else if self.head_pos.x < 0. as f32 {
            self.head_pos.x = WIDTH as f32 - SNAKE_SIZE
        } else if self.head_pos.y + SNAKE_SIZE > HEIGHT as f32 {
            self.head_pos.y = 0.
        } else if self.head_pos.y < 0. {
            self.head_pos.y = HEIGHT as f32 - SNAKE_SIZE
        }
        // Tail
        if self.tail_pos.x + SNAKE_SIZE > WIDTH as f32 {
            self.tail_pos.x = 0.
        } else if self.tail_pos.x < 0. as f32 {
            self.tail_pos.x = WIDTH as f32 - SNAKE_SIZE
        } else if self.tail_pos.y + SNAKE_SIZE > HEIGHT as f32 {
            self.tail_pos.y = 0.
        } else if self.tail_pos.y < 0. {
            self.tail_pos.y = HEIGHT as f32 - SNAKE_SIZE
        }
    }
    pub fn input_handling(&mut self) {
        if is_key_pressed(KeyCode::J) && self.head_dir.can_change_to(Direction::Down) {
            self.head_dir = Direction::Down
        } else if is_key_pressed(KeyCode::K) && self.head_dir.can_change_to(Direction::Up) {
            self.head_dir = Direction::Up
        } else if is_key_pressed(KeyCode::H) && self.head_dir.can_change_to(Direction::Left) {
            self.head_dir = Direction::Left
        } else if is_key_pressed(KeyCode::L) && self.head_dir.can_change_to(Direction::Right) {
            self.head_dir = Direction::Right
        }
    }
    pub fn update(&mut self) {
        // bug i can change to opposite direction if i pressed top/bottom and then the opposite direction
        //as an exmaple !
        // accept the input but enable changes to the snake only after update logic happend
        // must not update the snake pos between the frames

        let old_head_pos: Vec2 = self.head_pos.clone();
        self.tail_pos.x = old_head_pos.x;
        self.tail_pos.y = old_head_pos.y;
        match self.head_dir {
            Direction::Up => self.head_pos.y -= GRID_BOX,
            Direction::Down => self.head_pos.y += GRID_BOX,
            Direction::Right => self.head_pos.x += GRID_BOX,
            Direction::Left => self.head_pos.x -= GRID_BOX,
        }
        self.wall_collistion();
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.head_pos.x,
            self.head_pos.y,
            SNAKE_SIZE,
            SNAKE_SIZE,
            self.color,
        );
        draw_rectangle(
            self.tail_pos.x,
            self.tail_pos.y,
            SNAKE_SIZE,
            SNAKE_SIZE,
            BLUE,
        );
    }
}
