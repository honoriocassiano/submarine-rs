use sdl2::rect::Rect;

use crate::element::Element;
use crate::renderable::{RenderData, Renderable};
use crate::resources::ResourceLoader;

pub struct Player<'a> {
    render_data: RenderData<'a>,
}

impl<'a> Player<'a> {
    pub fn new() -> Player<'a> {
        let surface = ResourceLoader::load_image("assets/images/submarine.png");
        let rect = Rect::new(0, 0, 0, 0);
        let dest_rect = rect;

        let render_data = RenderData {
            surface,
            rect,
            dest_rect,
        };

        Player { render_data }
    }
}

impl<'a> Renderable<'a> for Player<'a> {
    fn data(&self) -> &RenderData<'a> {
        &self.render_data
    }
}

impl<'a> Element for Player<'a> {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self, _delta_time: f32) {
        todo!()
    }
}
