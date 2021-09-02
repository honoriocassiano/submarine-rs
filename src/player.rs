use sdl2::rect::Rect;

use crate::element::Element;
use crate::renderable::{RenderData, Renderable};
use crate::resources::ResourceLoader;
use crate::tree::RenderableElement;

pub struct Player {
    render_data: RenderData,
}

impl Player {
    pub fn new() -> Player {
        let texture = ResourceLoader::load_image("assets/images/submarine.png");
        let rect = Rect::new(0, 0, 0, 0);
        let dest_rect = rect;

        let render_data = RenderData {
            texture,
            rect,
            dest_rect,
        };

        Player { render_data }
    }
}

impl Renderable for Player {
    fn data(&self) -> &RenderData {
        &self.render_data
    }
}

impl Element for Player {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self, _delta_time: f32) {
        todo!()
    }
}
