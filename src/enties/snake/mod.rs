// use grids first, with the snake move on one direcetion
use crate::config::*;
pub mod direction;

use crate::Assests;
use direction::*;
use macroquad::prelude::*;

pub struct Snake<'a> {
    pub pos: Vec<Vec2>,
    head_dir: Direction,
    texture: &'a Assests,
}
impl<'a> Snake<'a> {
    pub fn new(pos: Vec<Vec2>, head_dir: Direction, texture: &'a Assests) -> Self {
        Self {
            pos,
            head_dir,
            texture,
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
    pub fn input_handling(&mut self) -> i32 {
        if is_key_pressed(KeyCode::J) && self.head_dir.can_change_to(Direction::Down) {
            self.head_dir = Direction::Down;
            1
        } else if is_key_pressed(KeyCode::K) && self.head_dir.can_change_to(Direction::Up) {
            self.head_dir = Direction::Up;
            1
        } else if is_key_pressed(KeyCode::H) && self.head_dir.can_change_to(Direction::Left) {
            self.head_dir = Direction::Left;
            1
        } else if is_key_pressed(KeyCode::L) && self.head_dir.can_change_to(Direction::Right) {
            self.head_dir = Direction::Right;
            1
        } else {
            0
        }
    }
    pub fn update(&mut self) {
        let mut old_cell_pos = self.pos[0];

        // Head
        match self.head_dir {
            Direction::Up => self.pos[0].y -= GRID_BOX,
            Direction::Down => self.pos[0].y += GRID_BOX,
            Direction::Right => self.pos[0].x += GRID_BOX,
            Direction::Left => self.pos[0].x -= GRID_BOX,
        }

        // Tail
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
    pub fn collision(&self) {
        for i in 1..self.pos.len() {
            if self.pos[0] == self.pos[i] {
                println!("snake head collide with his tail !");
                // TODO make like you Lose text or something
            }
        }
    }
    pub fn draw(&self) {
        for (i, cell) in self.pos.iter().enumerate() {
            if i == 0 {
                draw_rectangle(
                    cell.x,
                    cell.y,
                    SNAKE_SIZE,
                    SNAKE_SIZE,
                    Color {
                        r: 0.156863,
                        g: 0.360784,
                        b: 0.768627,
                        a: 1.0,
                    },
                );
                // draw_texture(&self.texture.tail_sprite, cell.x, cell.y, WHITE);
            } else {
                draw_rectangle(
                    cell.x,
                    cell.y,
                    SNAKE_SIZE,
                    SNAKE_SIZE,
                    Color {
                        r: 0.078431,
                        g: 0.203922,
                        b: 0.392157,
                        a: 1.0,
                    },
                );
                // draw_texture(&self.texture.haid_sprite, cell.x, cell.y, WHITE);
            }
        }
    }
}
/*
make gradient on snake body using the blue one, or it could be rainbow
make a score
make Ui struct ? and methods

- i have very nive ideas but i am on debt of trying to implemnet them
*/
