use macroquad::prelude::*;
pub struct Assests {
    pub head_sprite: Texture2D,
    pub tail_sprite: Texture2D,
    pub food_sprite: Texture2D,
}
impl Assests {
    pub async fn load() -> Self {
        Self {
            tail_sprite: load_texture("assets/snake_tail.png").await.unwrap(),
            head_sprite: load_texture("assets/snake_head.png").await.unwrap(),
            food_sprite: load_texture("assets/applev2.png").await.unwrap(),
        }
    }
}
