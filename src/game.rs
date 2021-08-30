use crate::sdl_system::SdlSystem;
use crate::window::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const NAME: &'static str = "Submarine";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Game {
    sdl_system: SdlSystem,
    window: Window,
}

impl Game {
    pub fn new() -> Result<Game, String> {
        let mut sdl_system = SdlSystem::init()?;
        let window = Window::new(&mut sdl_system, NAME, WIDTH, HEIGHT)?;

        Ok(Game { sdl_system, window })
    }

    pub fn run(&mut self) {
        let canvas = self.window.canvas();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let event_pump = self.sdl_system.event_pump();
        let mut i = 0;

        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
            // The rest of the game loop goes here...

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// TODO Implement Drop trait
