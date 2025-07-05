extern crate alloc;

use agb::display::object::Object;
use agb::display::GraphicsFrame;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::rc::Rc;

use crate::gameobject::GameObject;
use crate::observer::{Event, Listener, Observable};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

const NAME: &str = "movingstone";

pub struct MovingStone{
    sprite: Sprite,
    signals_out: Observable
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
            signals_out: Observable::new(String::from(NAME))
        }
    }
}

impl MovingStone {
    pub fn subscribe(&mut self, subscriber: Rc<Listener>, events: Vec<Event>) {
        for en in events {
            self.signals_out.subscribe(en, subscriber.clone());
        }
    }
}

impl GameObject for MovingStone {
    fn behave(&mut self) {
        if self.sprite.get_x()+BALL_SIZE <= 0 {
            self.signals_out.notify(String::from("reset"));
            self.sprite.set_x(agb::display::WIDTH);
        } else {
            self.sprite.update_pos(Direction::LEFT);
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.render(frame);
    }
}
