use crate::gameobject::GameObject;
use agb::{
    display::{
        object::{Object}
    },
};

pub struct Sprite <'a> {
    x: i32,
    y: i32,
    velocity: i32,
    pub object: Object <'a>,
}

impl <'a> Sprite <'a> {
    pub fn new(x: i32, y: i32, velocity: i32, object: Object<'a>) -> Sprite<'a>{
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
}

impl GameObject for Sprite<'_> {
    fn behave(&mut self) {}

    fn render(&mut self) {
        let x = self.get_x() as u16;
        let y = self.get_y() as u16;
        self.object.set_x(x).set_y(y).show();
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}