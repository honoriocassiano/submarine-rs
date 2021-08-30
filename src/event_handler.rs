use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::sdl_system::SdlSystem;

pub struct EventHandler {
    pump: EventPump,
}

pub enum EventProcessingStatus {
    Continue,
    Stop,
}

impl EventHandler {
    pub fn new(sdl_system: &SdlSystem) -> Result<EventHandler, String> {
        let pump = sdl_system.sdl_context().event_pump()?;

        Ok(EventHandler { pump })
    }

    pub fn handle_events(&mut self) -> EventProcessingStatus {
        for event in self.pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return EventProcessingStatus::Stop;
                }
                _ => {}
            }
        }

        EventProcessingStatus::Continue
    }
}
