use agb::display::object::Object;

use crate::gameobject::GameObject;
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

pub struct MovingStone <'a> {
    sprite: Sprite <'a>
}

impl <'a> MovingStone<'a> {
    pub fn new(object: Object<'a>) -> MovingStone<'a> {
        Self {
            sprite: Sprite::new(
                agb::display::WIDTH / 2 - BALL_SIZE/2,
                agb::display::HEIGHT / 2 - BALL_SIZE/2,
                1,
                object
            ),
        }
    }
}

impl GameObject for MovingStone<'_> {
    fn behave(&mut self) {
        if self.sprite.get_x()+BALL_SIZE <= 0 {
            self.sprite.set_x(agb::display::WIDTH);
        } else {
            self.sprite.update_pos(Direction::LEFT);
        }
    }

    fn render(&mut self) {
        self.sprite.render();
    }
}