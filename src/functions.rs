use crate::config::*;
// use crate::enties::{food::Food, snake::Snake};
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_height: HEIGHT,
        window_width: WIDTH,
        #[cfg(target_os = "linux")]
        platform: Platform {
            linux_wm_class: WINDOW_TITLE,
            ..Default::default()
        },
        ..Default::default()
    }
}
pub fn grid_draw() {
    for x in 0..=WIDTH {
        for y in 0..=WIDTH {
            if x % GRID_BOX as i32 == 0 && y % GRID_BOX as i32 == 0 {
                draw_line(x as f32, 0., x as f32, HEIGHT as f32, 1., GRAY);
                draw_line(0., y as f32, WIDTH as f32, y as f32, 1., GRAY);
            }
        }
    }
}
pub fn random_spot(dim: i32) -> f32 {
    let mut pos = rand::gen_range(0, dim);
    // how much do i need to make that range multipliable by 40 ?
    let result_dev_x = pos % 40;

    pos -= result_dev_x;
    pos as f32
}
