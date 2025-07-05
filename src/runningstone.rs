extern crate alloc;

use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::fixnum::{vec2, Rect};
use alloc::rc::Rc;

use crate::gameobject::GameObject;
use crate::observer::{Event, Listener};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

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
                Event::Position(r) => {
                    if r.touches(Rect::new(vec2(self.sprite.get_x(), self.sprite.get_y()), vec2(BALL_SIZE, BALL_SIZE))) {
                        for _ in 0..5 {
                            self.sprite.update_pos(Direction::DOWN);
                        }
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
