extern crate sdl2;

use crate::game::Game;

mod game;
mod sdl_system;
mod window;

pub fn main() {
    let mut game = Game::new().unwrap();

    game.run();
}
