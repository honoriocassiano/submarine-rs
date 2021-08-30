use std::time::Duration;

use crate::event_handler::{EventHandler, EventProcessingStatus};
use crate::sdl_system::SdlSystem;
use crate::window::Window;

const NAME: &'static str = "Submarine";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Game {
    sdl_system: SdlSystem,
    window: Window,
    event_handler: EventHandler,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let mut sdl_system = SdlSystem::init().unwrap();
        let window = Window::new(&mut sdl_system, NAME, WIDTH, HEIGHT).unwrap();
        let event_handler = EventHandler::new(&sdl_system).unwrap();

        Ok(Game {
            sdl_system,
            window,
            event_handler,
        })
    }

    pub fn run(&mut self) {
        self.window.init();

        'running: loop {
            match self.event_handler.handle_events() {
                EventProcessingStatus::Continue => {}
                EventProcessingStatus::Stop => {
                    break 'running;
                }
            }

            self.window.update();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// TODO Implement Drop trait
