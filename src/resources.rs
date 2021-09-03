use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

pub struct ResourceLoader {}

impl ResourceLoader {
    // TODO Use Path as parameter type
    pub fn load_image(filename: &'static str) -> Surface<'_> {
        Surface::from_file(filename).unwrap()
    }
}
