use std::collections::HashMap;
use std::rc::Rc;

use sdl2::image::LoadSurface;
use sdl2::mixer::Music;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

pub struct ResourceManager<'a> {
    resources: HashMap<&'static str, Surface<'a>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            resources: HashMap::default(),
        })
    }

    pub fn load_image(&mut self, filename: &'static str) -> &Surface<'a> {
        let surface = Surface::from_file(filename).unwrap();

        self.resources.insert(filename, surface).unwrap();

        self.resources.get(filename).unwrap()
    }

    pub fn load_audio(&mut self, _filename: &'static str) -> Rc<Music<'a>> {
        todo!()
    }

    pub fn load_font<'b>(&mut self, _filename: &'static str) -> Rc<Font<'a, 'b>> {
        todo!()
    }
}
