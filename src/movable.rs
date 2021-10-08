use crate::vec2::Vec2;

trait Positionable {
    fn position(&self) -> Vec2;
}

trait Movable: Positionable {
    fn move_by(&mut self, d_position: Vec2);

    fn move_to(&mut self, new_position: Vec2);
}
