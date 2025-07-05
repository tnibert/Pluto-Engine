extern crate alloc;

use crate::gameobject::GameObject;
use crate::observer::{Observable, Listener, Event};
use crate::sprite::{Sprite, Direction};
use crate::BALL_SIZE;
use agb::display::object::Object;
use agb::display::GraphicsFrame;
use agb::input::{Button, ButtonController};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

const NAME: &str = "player";

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
                agb::display::WIDTH / 2 - BALL_SIZE/2,
                agb::display::HEIGHT / 2 - BALL_SIZE/2,
                1,
                object
            ),
            input: ButtonController::new(),  // supposedly I can create two of these, should test how it behaves in practice
            observer: Rc::new(Listener::new()),
            signals_out: Observable::new(String::from(NAME))
        }
    }

    pub fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }

    pub fn subscribe(&mut self, subscriber: Rc<Listener>, events: Vec<Event>) {
        for en in events {
            self.signals_out.subscribe(en, subscriber.clone());
        }
    }
}

impl GameObject for Player {
    fn behave(&mut self) {
        self.input.update();

        if self.input.is_pressed(Button::UP) && self.sprite.get_y() > 0 {
            self.sprite.update_pos(Direction::UP);
            self.signals_out.notify(Event::Position);
        }
        if self.input.is_pressed(Button::DOWN) && self.sprite.get_y() < agb::display::HEIGHT - BALL_SIZE {
            self.sprite.update_pos(Direction::DOWN);
            self.signals_out.notify(Event::Position);
        }
        if self.input.is_pressed(Button::LEFT) && self.sprite.get_x() > 0 {
            self.sprite.update_pos(Direction::LEFT);
            self.signals_out.notify(Event::Position);
        }
        if self.input.is_pressed(Button::RIGHT) && self.sprite.get_x() < agb::display::WIDTH - BALL_SIZE {
            self.sprite.update_pos(Direction::RIGHT);
            self.signals_out.notify(Event::Position);
        }

        // check event subscriptions
        for e in self.observer.poll_evt() {
            match e {
                Event::Reset => {
                    for _ in 0..10 {
                        // just a test to make sure observer works
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