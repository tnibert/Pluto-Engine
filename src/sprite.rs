use crate::gameobject::GameObject;
use agb::{
    display::{
        object::{Object}
    },
};
use agb::fixnum::vec2;
use agb::display::GraphicsFrame;

pub struct Sprite {
    x: i32,
    y: i32,
    velocity: i32,
    pub object: Object,
}

impl Sprite {
    pub fn new(x: i32, y: i32, velocity: i32, object: Object) -> Sprite{
        Self {
            x: x,
            y: y,
            velocity: velocity,
            object: object,
        }
    }

    pub fn update_pos(&mut self, dir: Direction) {
        match dir {
            Direction::LEFT => self.x -= self.velocity,
            Direction::RIGHT => self.x += self.velocity,
            Direction::UP => self.y -= self.velocity,
            Direction::DOWN => self.y += self.velocity
        }
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_velocity(&mut self, velocity: i32) {
        self.velocity = velocity;
    }
}

impl GameObject for Sprite {
    fn behave(&mut self) {}

    fn render(&mut self, frame: &mut GraphicsFrame) {
        let x = self.get_x();
        let y = self.get_y();
        let pos = vec2(x, y);
        self.object.set_pos(pos).show(frame);
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}