use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

pub struct ResourceLoader {}

impl ResourceLoader {
    pub fn load_image(filename: &'static str) -> Surface<'_> {
        let surface = Surface::from_file(filename).unwrap();

        surface
    }
}
