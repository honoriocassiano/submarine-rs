use std::time::Duration;

use crate::element::Element;
use crate::event_handler::{EventHandler, EventProcessingStatus};
use crate::player::Player;
use crate::renderer::Renderer;
use crate::resources::ResourceLoader;
use crate::sdl_system::SdlSystem;
use crate::tree::Tree;
use crate::window::Window;

const NAME: &'static str = "Submarine";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Game {
    sdl_system: SdlSystem,
    window: Window,
    event_handler: EventHandler,
    tree: Tree,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let mut sdl_system = SdlSystem::init().unwrap();
        let mut window = Window::new(&mut sdl_system, NAME, WIDTH, HEIGHT).unwrap();
        let event_handler = EventHandler::new(&sdl_system).unwrap();
        let tree = Game::init_tree(&mut window.renderer_mut());

        Ok(Self {
            sdl_system,
            window,
            event_handler,
            tree,
        })
    }

    fn init_tree(renderer: &mut Renderer) -> Tree {
        let surface = ResourceLoader::load_image("assets/images/submarine.png");

        let player = Box::new(Player::new(renderer.load_texture(surface)));

        let mut tree = Tree::new();

        tree.add_child(player);

        tree
    }

    pub fn run(&mut self) {
        self.tree.init();

        'running: loop {
            match self.event_handler.handle_events() {
                EventProcessingStatus::Continue => {}
                EventProcessingStatus::Stop => {
                    break 'running;
                }
            }

            // TODO Add timer
            self.tree.update(0.0f32);
            self.window.render(&self.tree);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// TODO Implement Drop trait
