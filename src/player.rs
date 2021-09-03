use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use crate::element::Element;
use crate::renderable::{RenderData, Renderable};
use crate::tree::RenderableElement;

pub struct Player {
    render_data: RenderData,
}

impl Player {
    pub fn new(texture: Rc<Texture>) -> Player {
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
    fn init(&mut self) {}

    fn update(&mut self, _delta_time: f32) {}
}

impl RenderableElement for Player {}
