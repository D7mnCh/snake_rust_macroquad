pub mod direction;

use crate::assets::*;
use crate::config::Config;
use crate::traits::*;
use crate::game_state::GameState;
use direction::Direction;
use macroquad::{
    input::{KeyCode, is_key_pressed},
    math::Vec2,
    texture::draw_texture,
    color::colors::WHITE
};

pub struct Snake {
    pub pos: Vec<Vec2>,
    pub size: f32,
    pub head_dir: Direction,
    pub score: i32,
    head_sprite: SpriteId,
    tail_sprite: SpriteId,
}
impl Snake {
    pub fn new(config: &Config) -> Self {
        let mut pos = Vec::new();
        let size = config.grid_box;
        let num_of_cell = 2;
        for i in 0..=num_of_cell {
            let new_cell = Vec2::new(
                config.screen_width as f32 / 2.,
                config.screen_height as f32 / 2. + (config.grid_box as i32 * i) as f32,
            );
            pos.push(new_cell);
        }
        let head_sprite = SpriteId::HeadSprite;
        let tail_sprite = SpriteId::TailSprite;
        let head_dir = Direction::Up;
        let score = 0;

        Self {
            pos,
            size,
            head_dir,
            head_sprite,
            tail_sprite,
            score,
        }
    }
    fn wall_collistion(&mut self, config: &Config) {
        for cell in self.pos.iter_mut() {
            if cell.x + self.size > config.screen_width as f32 {
                cell.x = 0.
            } else if cell.x < 0. as f32 {
                cell.x = config.screen_width as f32 - self.size
            } else if cell.y + self.size > config.screen_height as f32 {
                cell.y = 0.
            } else if cell.y < 0. {
                cell.y = config.screen_height as f32 - self.size
            }
        }
    }
    // should break this method
    pub fn input_handling(&mut self) {
        if (is_key_pressed(KeyCode::J) || is_key_pressed(KeyCode::S))
            && self.head_dir.can_change_to(Direction::Down)
        {
            self.head_dir = Direction::Down;
        }
        if (is_key_pressed(KeyCode::K) || is_key_pressed(KeyCode::W))
            && self.head_dir.can_change_to(Direction::Up)
        {
            self.head_dir = Direction::Up;
        }
        if (is_key_pressed(KeyCode::H) || is_key_pressed(KeyCode::A))
            && self.head_dir.can_change_to(Direction::Left)
        {
            self.head_dir = Direction::Left;
        }
        if (is_key_pressed(KeyCode::L) || is_key_pressed(KeyCode::D))
            && self.head_dir.can_change_to(Direction::Right)
        {
            self.head_dir = Direction::Right;
        }
    }
    pub fn reset(&mut self, config: &Config) {
        self.head_dir = Direction::Up;
        self.score = 0;
        self.pos.clear();
        for i in 0..=2 {
            let new_cell = Vec2::new(
                config.screen_width as f32 / 2.,
                config.screen_height as f32 / 2. + (config.grid_box as i32 * i) as f32,
            );
            self.pos.push(new_cell);
        }
    }
    pub fn update(&mut self, config: &Config) {
        let mut old_cell_pos = self.pos[0];

        // Head
        match self.head_dir {
            Direction::Up => self.pos[0].y -= config.grid_box as f32,
            Direction::Down => self.pos[0].y += config.grid_box as f32,
            Direction::Right => self.pos[0].x += config.grid_box as f32,
            Direction::Left => self.pos[0].x -= config.grid_box as f32,
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
        self.wall_collistion(&config);
    }

    pub fn check_collision_detection(&mut self) -> bool {
        for i in 1..self.pos.len() {
            if self.pos[0] == self.pos[i] {
                return true;
            }
        }
        false
    }

    pub fn grow(&mut self) {
        let new_snake_cell = Vec2::new(
            self.pos[self.pos.len() - 1].x,
            self.pos[self.pos.len() - 1].y,
        );
        self.pos.push(new_snake_cell);
    }
}

impl Renderable for Snake {
    async fn draw(&self, assets: &Assets) {
        let head_texture = self.head_sprite.get_texture(assets).await;
        let tail_texture = self.tail_sprite.get_texture(assets).await;
        for cell in self.pos.iter() {
            draw_texture(tail_texture, cell.x, cell.y, WHITE);
        }
        draw_texture(head_texture, self.pos[0].x, self.pos[0].y, WHITE);
    }
}
impl Updatable for Snake {
    fn update(&mut self, config: &Config) {
        let mut old_cell_pos = self.pos[0];

        // Head
        match self.head_dir {
            Direction::Up => self.pos[0].y -= config.grid_box as f32,
            Direction::Down => self.pos[0].y += config.grid_box as f32,
            Direction::Right => self.pos[0].x += config.grid_box as f32,
            Direction::Left => self.pos[0].x -= config.grid_box as f32,
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
    }
}
