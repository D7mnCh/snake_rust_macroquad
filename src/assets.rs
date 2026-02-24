use macroquad::prelude::*;
pub enum SpriteId {
    HeadSprite,
    TailSprite,
    FoodSprite,
}
impl SpriteId {
    pub async fn get_texture<'a>(&self, assets: &'a Assets) -> &'a Texture2D {
        match self {
            Self::HeadSprite => &assets.head_sprite,
            Self::TailSprite => &assets.head_sprite,
            Self::FoodSprite => &assets.head_sprite,
        }
    }
}
pub struct Assets {
    pub head_sprite: Texture2D,
    pub tail_sprite: Texture2D,
    pub food_sprite: Texture2D,
}
impl Assets {
    pub async fn load() -> Self {
        Self {
            tail_sprite: load_texture("assets/snake_tail.png").await.unwrap(),
            head_sprite: load_texture("assets/snake_head.png").await.unwrap(),
            food_sprite: load_texture("assets/yellow_apple.png").await.unwrap(),
        }
    }
}
