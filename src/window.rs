use crate::renderer::Renderer;
use crate::sdl_system::SdlSystem;
use crate::tree::Tree;

pub struct Window {
    renderer: Renderer,
}

impl Window {
    pub fn new(
        sdl_system: &mut SdlSystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
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

    pub fn render(&mut self, tree: &Tree) {
        self.renderer.render(tree);
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    pub fn renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
}

// TODO Implement Drop trait
