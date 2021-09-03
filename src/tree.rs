use crate::element::Element;
use crate::renderable::Renderable;

pub trait RenderableElement: Element + Renderable {}

pub struct Tree {
    elements: Vec<Box<dyn RenderableElement>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_child(&'_ mut self, element: Box<dyn RenderableElement>) {
        self.elements.push(element);
    }

    pub fn children(&self) -> &Vec<Box<dyn RenderableElement>> {
        &self.elements
    }

    pub fn children_mut(&mut self) -> &mut Vec<Box<dyn RenderableElement>> {
        &mut self.elements
    }
}

impl Element for Tree {
    fn init(&mut self) {
        for e in self.elements.iter_mut() {
            e.init();
        }
    }

    fn update(&mut self, delta_time: f32) {
        for e in self.elements.iter_mut() {
            e.update(delta_time);
        }
    }
}
