use crate::assets::*;
use crate::config::Config;
pub trait Renderable {
    async fn draw(&self, assets: &Assets);
}
pub trait Updatable {
    fn update(&mut self, config: &Config);
}
