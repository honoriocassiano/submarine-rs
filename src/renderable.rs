use sdl2::rect::Rect;
use sdl2::surface::Surface;

pub struct RenderData<'a> {
    pub surface: Surface<'a>,
    pub rect: Rect,
    pub dest_rect: Rect,
}

pub trait Renderable<'a> {
    fn data(&self) -> &RenderData<'a>;
}
