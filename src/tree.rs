use crate::element::Element;

pub struct Tree {
    elements: Vec<Box<dyn Element>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_child(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }

    pub fn children(&mut self) -> &mut Vec<Box<dyn Element>> {
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
