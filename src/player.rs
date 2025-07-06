extern crate alloc;

use crate::gameobject::GameObject;
use crate::observer::{Event, Listener, Observable, Publisher, Subscriber};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;
use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::fixnum::{vec2, Rect};
use agb::input::{Button, ButtonController};
use alloc::rc::Rc;
use alloc::string::String;

const NAME: &str = "player";
const ORIGINAL_X: i32 = agb::display::WIDTH / 2 - BALL_SIZE/2;
const ORIGINAL_Y: i32 = agb::display::HEIGHT - BALL_SIZE;

pub struct Player {
    sprite: Sprite,
    input: ButtonController,
    observer: Rc<Listener>,
    signals_out: Observable
}

impl Player {
    pub fn new(object: Object) -> Player {
        Self {
            sprite: Sprite::new(
                ORIGINAL_X,
                ORIGINAL_Y,
                1,
                object
            ),
            input: ButtonController::new(),  // supposedly I can create two of these, should test how it behaves in practice
            observer: Rc::new(Listener::new()),
            signals_out: Observable::new(String::from(NAME))
        }
    }
}

impl GameObject for Player {
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

        //agb::println!("player sending position");
        self.signals_out.notify(Event::Position(Rect::new(vec2(self.sprite.get_x(), self.sprite.get_y()), vec2(BALL_SIZE, BALL_SIZE))));

        // check event subscriptions
        for e in self.observer.poll_evt() {
            match e {
                Event::Position(r) => {
                    if r.touches(Rect::new(vec2(self.sprite.get_x(), self.sprite.get_y()), vec2(BALL_SIZE, BALL_SIZE))) {
                        self.sprite.set_x(ORIGINAL_X);
                        self.sprite.set_y(ORIGINAL_Y);
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

impl Publisher for Player {
    fn register_subscription(&mut self, subscriber: Rc<Listener>, event: Event) {
        self.signals_out.register_subscription(subscriber, event);
    }
}

impl Subscriber for Player {
    fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }
}
