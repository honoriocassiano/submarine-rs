use crate::element::Element;
use crate::renderer::Renderer;
use crate::sdl_system::SdlSystem;

pub struct Window {
    renderer: Renderer,
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
        let renderer = Renderer::new(canvas);

        Ok(Window { renderer })
    }
}

impl Element for Window {
    fn init(&mut self) {
        self.renderer.init();
    }

    fn update(&mut self) {
        self.renderer.update();
    }
}

// TODO Implement Drop trait
