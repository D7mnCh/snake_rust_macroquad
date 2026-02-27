mod app;
mod assets;
mod config;
mod entities;
mod functions;
mod game_state;
mod systems;
mod traits;
mod ui;
use crate::app::App;
use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game: App = App::new().await;
    game.run().await;
}
