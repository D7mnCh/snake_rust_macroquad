use crate::assets::*;
pub trait Renderable {
    async fn draw(&self, assets: &Assets);
}
pub trait Updatable {
    fn update(&mut self);
}
