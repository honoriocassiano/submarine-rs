use std::rc::Rc;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use crate::tree::Tree;

pub struct Renderer {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<Rc<Texture>>,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas) -> Self {
        let texture_creator = canvas.texture_creator();
        let textures = Vec::new();

        Self {
            canvas,
            texture_creator,
            textures,
        }
    }

    pub fn load_texture(&mut self, surface: Surface) -> Rc<Texture> {
        let texture = Rc::new(
            self.texture_creator
                .create_texture_from_surface(&surface)
                .unwrap(),
        );

        self.textures.push(texture.clone());

        texture
    }

    pub fn render(&mut self, tree: &Tree) {
        for element in tree.children() {
            let data = element.data();

            // TODO Use this value
            self.canvas.copy(&data.texture, data.rect, data.dest_rect);
        }
    }
}
