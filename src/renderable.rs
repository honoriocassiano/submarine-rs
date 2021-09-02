use sdl2::rect::Rect;
use sdl2::surface::Surface;

pub struct RenderData {
    pub surface: Surface<'static>,
    pub rect: Rect,
    pub dest_rect: Rect,
}

pub trait Renderable {
    fn data(&self) -> &RenderData;
}
