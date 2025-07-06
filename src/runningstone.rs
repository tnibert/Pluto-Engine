extern crate alloc;

use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::fixnum::{vec2, Rect};
use alloc::rc::Rc;
use alloc::string::String;

use crate::gameobject::GameObject;
use crate::math::random_constrained_positive;
use crate::observer::{Event, Listener, Observable, Publisher, Subscriber};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

const NAME: &str = "running stone";

pub struct RunningStone{
    sprite: Sprite,
    observer: Rc<Listener>,
    observable: Observable
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
            observer: Rc::new(Listener::new()),
            observable: Observable::new(String::from(NAME))
        }
    }
}

impl GameObject for RunningStone {
    fn behave(&mut self) {
        // check event subscriptions
        for e in self.observer.poll_evt() {
            match e {
                Event::Position(r) => {
                    //agb::println!("running stone received position");
                    if r.touches(Rect::new(vec2(self.sprite.get_x(), self.sprite.get_y()), vec2(BALL_SIZE, BALL_SIZE))) {
                        for _ in 0..5 {
                            self.sprite.update_pos(Direction::DOWN);
                        }
                    }
                },
                _ => ()
            }
        }

        // notify if running stone has been retrieved to bottom of the screen
        if self.sprite.get_y() >= agb::display::HEIGHT {
            let new_x_pos = random_constrained_positive(agb::display::WIDTH - BALL_SIZE);
            self.sprite.set_y(0);
            self.sprite.set_x(new_x_pos);

            self.observable.notify(Event::Reset);
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.render(frame);
    }
}

impl Subscriber for RunningStone {
    fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }
}

impl Publisher for RunningStone {
    fn register_subscription(&mut self, subscriber: Rc<Listener>, evt: Event) {
        self.observable.register_subscription(subscriber, evt);
    }
}
