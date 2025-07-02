use crate::gameobject::GameObject;
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;
use agb::display::object::Object;
use agb::input::{Button, ButtonController};

pub struct Player <'a> {
    sprite: Sprite <'a>,
    input: ButtonController
}

impl <'a> Player<'a> {
    pub fn new(object: Object<'a>) -> Player<'a> {
        Self {
            sprite: Sprite::new(
                agb::display::WIDTH / 2 - BALL_SIZE/2,
                agb::display::HEIGHT / 2 - BALL_SIZE/2,
                1,
                object
            ),
            input: ButtonController::new()  // supposedly I can create two of these, should test how it behaves in practice
        }
    }
}

impl GameObject for Player<'_> {
    fn behave(&mut self) {
        self.input.update();

        if self.input.is_pressed(Button::UP) && self.sprite.get_y() > 0 {
            self.sprite.update_pos(Direction::UP);
        }
        if self.input.is_pressed(Button::DOWN) && self.sprite.get_y() < agb::display::HEIGHT - BALL_SIZE {
            self.sprite.update_pos(Direction::DOWN);
        }
        if self.input.is_pressed(Button::LEFT) && self.sprite.get_x() > 0 {
            self.sprite.update_pos(Direction::LEFT);
        }
        if self.input.is_pressed(Button::RIGHT) && self.sprite.get_x() < agb::display::WIDTH - BALL_SIZE {
            self.sprite.update_pos(Direction::RIGHT);
        }
    }

    fn render(&mut self) {
        self.sprite.render();
    }
}