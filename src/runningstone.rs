extern crate alloc;

use agb::display::object::Object;
use agb::display::GraphicsFrame;
use alloc::rc::Rc;

use crate::gameobject::GameObject;
use crate::observer::{Event, Listener};
use crate::sprite::{Sprite, Direction};

pub struct RunningStone{
    sprite: Sprite,
    observer: Rc<Listener>
}

impl RunningStone{
    pub fn new(x_start: i32, y_start: i32, object: Object) -> RunningStone {
        Self {
            sprite: Sprite::new(
                x_start,
                y_start,
                1,
                object
            ),
            observer: Rc::new(Listener::new())
        }
    }
}

impl RunningStone {
    pub fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }
}

impl GameObject for RunningStone {
    fn behave(&mut self) {
        // check event subscriptions
        for e in self.observer.poll_evt() {
            match e {
                Event::Position => {
                    for _ in 0..5 {
                        self.sprite.update_pos(Direction::DOWN);
                    }
                },
                _ => ()
            }
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.render(frame);
    }
}
