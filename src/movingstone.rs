extern crate alloc;

use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::fixnum::{vec2, Rect};
use alloc::string::String;
use alloc::rc::Rc;

use crate::gameobject::GameObject;
use crate::math::random_constrained_positive;
use crate::observer::{Event, Listener, Observable, Publisher};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;

const NAME: &str = "movingstone";
const MAX_VELOCITY: i32 = 3;

pub struct MovingStone{
    sprite: Sprite,
    signals_out: Observable
}

impl MovingStone{
    pub fn new(x_start: i32, y_start: i32, object: Object) -> MovingStone {
        let mut velocity = random_constrained_positive(MAX_VELOCITY);
        if velocity == 0 {
            velocity = 1;
        }
        Self {
            sprite: Sprite::new(
                x_start,
                y_start,
                velocity,
                object
            ),
            signals_out: Observable::new(String::from(NAME))
        }
    }
}

impl GameObject for MovingStone {
    fn behave(&mut self) {
        if self.sprite.get_x()+BALL_SIZE <= 0 {
            self.signals_out.notify(Event::Reset);
            self.sprite.set_x(agb::display::WIDTH);
            let mut new_velocity = random_constrained_positive(MAX_VELOCITY);
            if new_velocity == 0 {
                new_velocity = 1;
            }
            self.sprite.set_velocity(new_velocity);
        } else {
            self.sprite.update_pos(Direction::LEFT);
        }
        self.signals_out.notify(Event::Position(Rect::new(vec2(self.sprite.get_x(), self.sprite.get_y()), vec2(BALL_SIZE, BALL_SIZE))));
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.render(frame);
    }
}

impl Publisher for MovingStone {
    fn register_subscription(&mut self, subscriber: Rc<Listener>, event: Event) {
        self.signals_out.register_subscription(subscriber, event);
    }
}
