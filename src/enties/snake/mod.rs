// use grids first, with the snake move on one direcetion
use crate::config::*;
pub mod direction;

use direction::*;
use macroquad::prelude::*;

pub struct Snake {
    pub pos: Vec<Vec2>,
    head_dir: Direction,
    color: Color,
}
impl Snake {
    pub fn new(pos: Vec<Vec2>, head_dir: Direction, color: Color) -> Self {
        Self {
            pos,
            head_dir,
            color,
        }
    }
    fn wall_collistion(&mut self) {
        for cell in self.pos.iter_mut() {
            if cell.x + SNAKE_SIZE > WIDTH as f32 {
                cell.x = 0.
            } else if cell.x < 0. as f32 {
                cell.x = WIDTH as f32 - SNAKE_SIZE
            } else if cell.y + SNAKE_SIZE > HEIGHT as f32 {
                cell.y = 0.
            } else if cell.y < 0. {
                cell.y = HEIGHT as f32 - SNAKE_SIZE
            }
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
        //as an exmaple ! (need to be fast a little)
        // accept the input but enable changes to the snake only after update logic happend
        // must not update the snake pos between the frames
        let mut old_cell_pos = self.pos[0];

        // Head
        match self.head_dir {
            Direction::Up => self.pos[0].y -= GRID_BOX,
            Direction::Down => self.pos[0].y += GRID_BOX,
            Direction::Right => self.pos[0].x += GRID_BOX,
            Direction::Left => self.pos[0].x -= GRID_BOX,
        }

        for i in 0..self.pos.len() {
            if i != 0 {
                // i need to store the postion of the current cell first
                let current_cell_pos = self.pos[i];
                self.pos[i].x = old_cell_pos.x;
                self.pos[i].y = old_cell_pos.y;
                old_cell_pos = current_cell_pos;
            }
        }
        self.wall_collistion();
    }
    pub fn draw(&self) {
        for cell in self.pos.iter() {
            draw_rectangle(cell.x, cell.y, SNAKE_SIZE, SNAKE_SIZE, BLUE);
        }
    }
}
