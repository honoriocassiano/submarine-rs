pub trait Element {
    fn init(&mut self);

    fn update(&mut self, delta_time: f32);
}
