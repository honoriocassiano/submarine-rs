use std::rc::Rc;

use sdl2::mixer::Music;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

enum ResourceType {
    Audio(&'static str),
    Font(&'static str),
    Image(&'static str),
    Music(&'static str),
}

trait Resource {}

impl Resource for Surface<'_> {}

impl Resource for Music<'_> {}

impl Resource for Font<'_, '_> {}

struct ResourceManager {}

impl ResourceManager {
    pub fn new() -> Result<ResourceManager, String> {
        Ok(ResourceManager {})
    }

    pub fn load(resource: ResourceType) -> Rc<dyn Resource> {
        todo!()
    }
}
