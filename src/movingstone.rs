use agb::display::object::Object;
use agb::display::GraphicsFrame;

use crate::gameobject::GameObject;
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

pub struct MovingStone{
    sprite: Sprite
}

impl MovingStone{
    pub fn new(x_start: i32, y_start: i32, object: Object) -> MovingStone {
        Self {
            sprite: Sprite::new(
                x_start,
                y_start,
                1,
                object
            ),
        }
    }
}

impl GameObject for MovingStone {
    fn behave(&mut self) {
        if self.sprite.get_x()+BALL_SIZE <= 0 {
            self.sprite.set_x(agb::display::WIDTH);
        } else {
            self.sprite.update_pos(Direction::LEFT);
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.render(frame);
    }
}
