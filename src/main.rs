mod app;
mod assets;
mod config;
mod entities;
mod functions;
mod traits;
mod ui;
mod game_state;
mod systems;

use crate::app::*;
use crate::functions::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game: App = App::new().await;
    game.run().await;
}
