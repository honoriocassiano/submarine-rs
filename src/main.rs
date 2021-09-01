extern crate sdl2;

use crate::game::Game;

mod element;
mod event_handler;
mod game;
mod player;
mod renderable;
mod renderer;
mod resources;
mod sdl_system;
mod window;

pub fn main() {
    let mut game = Game::new().unwrap();

    game.run();
}
