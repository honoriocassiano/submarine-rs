use std::rc::Rc;

use crate::element::Element;

pub trait TreeElement: Element {
    fn add_child(&mut self, element: Box<dyn TreeElement>);

    fn children(&mut self) -> &mut Vec<Box<dyn TreeElement>>;
}

pub struct Tree {
    elements: Vec<Box<dyn TreeElement>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
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

impl TreeElement for Tree {
    fn add_child(&mut self, element: Box<dyn TreeElement>) {
        self.elements.push(element);
    }

    fn children(&mut self) -> &mut Vec<Box<dyn TreeElement>> {
        &mut self.elements
    }
}
