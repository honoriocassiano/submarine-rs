extern crate sdl2;

use crate::game::Game;

mod event_handler;
mod game;
mod renderer;
mod sdl_system;
mod window;

pub fn main() {
    let mut game = Game::new().unwrap();

    game.run();
}
