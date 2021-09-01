use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use crate::renderable::{RenderData, Renderable};

pub struct Player<'a> {
    render_data: RenderData<'a>,
}

impl<'a> Player<'a> {
    pub fn new() -> Player<'a> {
        // TODO Load image instead
        let surface = Surface::new(0, 0, PixelFormatEnum::RGBA8888).unwrap();
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
