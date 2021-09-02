use std::time::Duration;

use crate::element::Element;
use crate::event_handler::{EventHandler, EventProcessingStatus};
use crate::resources::ResourceManager;
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
    resource_manager: ResourceManager<'static>,
    tree: Tree,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let mut sdl_system = SdlSystem::init().unwrap();
        let window = Window::new(&mut sdl_system, NAME, WIDTH, HEIGHT).unwrap();
        let event_handler = EventHandler::new(&sdl_system).unwrap();
        let resource_manager = ResourceManager::new().unwrap();
        let tree = Tree::new();

        Ok(Game {
            sdl_system,
            window,
            event_handler,
            resource_manager,
            tree,
        })
    }

    pub fn run(&mut self) {
        self.window.init();
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
            self.window.update(0.0f32);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// TODO Implement Drop trait
