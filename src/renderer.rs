use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::element::Element;

pub struct Renderer {
    canvas: WindowCanvas,
    counter: u8, // TODO Remove this field
}

impl Renderer {
    pub fn new(canvas: WindowCanvas) -> Renderer {
        let counter = 0;

        Renderer { canvas, counter }
    }
}

impl Element for Renderer {
    fn init(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    fn update(&mut self) {
        self.counter = (self.counter + 1) % 255;

        self.canvas
            .set_draw_color(Color::RGB(self.counter, 64, 255 - self.counter));
        self.canvas.clear();
        self.canvas.present();
    }
}
