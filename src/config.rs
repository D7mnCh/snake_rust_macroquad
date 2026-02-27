use macroquad::miniquad::conf::{Conf, Platform};
// put those two inside there structs !
pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,
    pub window_title: &'static str,
    pub grid_box: f32,
    pub target_fps: f32,
}

impl Config {
    pub fn new() -> Self {
        Self {
            screen_width: 800,
            screen_height: 800,
            window_title: "Snake",
            grid_box: 40.,
            target_fps: 1. / 10.,
        }
    }
}

pub fn window_conf() -> Conf {
    let config = Config::new();
    Conf {
        window_title: String::from(config.window_title),
        window_height: config.screen_height,
        window_width: config.screen_width,
        #[cfg(target_os = "linux")]
        platform: Platform {
            linux_wm_class: config.window_title,
            ..Default::default()
        },
        ..Default::default()
    }
}
