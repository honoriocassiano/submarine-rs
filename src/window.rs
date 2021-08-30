use sdl2::render::WindowCanvas;

use crate::sdl_system::SdlSystem;

pub struct Window {
    canvas: WindowCanvas,
}

impl Window {
    pub fn new(
        sdl_system: &mut SdlSystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Window, String> {
        let sdl_window = sdl_system
            .video()
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap(); // TODO Handle this error

        // TODO Check for more options
        let canvas = sdl_window.into_canvas().build().unwrap(); // TODO Handle this error

        Ok(Window { canvas })
    }

    pub fn canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}

// TODO Implement Drop trait
