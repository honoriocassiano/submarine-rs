use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct RenderData {
    pub texture: Texture<'static>,
    pub rect: Rect,
    pub dest_rect: Rect,
}

pub trait Renderable {
    fn data(&self) -> &RenderData;
}
