extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::rc::Rc;
use agb::{display::{GraphicsFrame, object::Object}, fixnum::{vec2, Rect}};

use crate::{
    agb_sprites,
    background::Background,
    gameobject::GameObject,
    movingstone::MovingStone,
    observer::{Event, Listener, Publisher, Subscriber},
    player::Player,
    runningstone::RunningStone,
    BALL_SIZE
};

pub struct Scene {
    gameobjects: Vec<Box<dyn GameObject>>,
    observer: Rc<Listener>
}

impl Scene {
    // setup the scene
    pub fn new() -> Scene {
        let scene_observer = Rc::new(Listener::new());

        let ball = Object::new(agb_sprites::BALL.sprite(0));
        let paddle_mid = Object::new(agb_sprites::PADDLE_MID.sprite(0));
        let paddle_end = Object::new(agb_sprites::PADDLE_END.sprite(0));

        // Create game objects
        let mut gameobjects: Vec<Box<dyn GameObject>> = Vec::new();
        let mut player = Box::new(Player::new(ball));
        let mut running_stone = Box::new(RunningStone::new(20, 20, paddle_end));

        for i in 1..9 {
            let mut moving_stone = Box::new(MovingStone::new(BALL_SIZE * i, BALL_SIZE * i, paddle_mid.clone()));
            moving_stone.register_subscription(player.observer(), Event::Position(Rect::new(vec2(0, 0), vec2(0, 0))));
            gameobjects.push(moving_stone);
        }
        // set up communication pathways
        // running stone listens for position from player
        player.register_subscription(running_stone.observer(), Event::Position(Rect::new(vec2(0, 0), vec2(0, 0))));
        // scene listens for reset from running stone
        running_stone.register_subscription(scene_observer.clone(), Event::Reset);

        // add gameobjects to the scene graph
        gameobjects.push(player);
        gameobjects.push(running_stone);

        gameobjects.push(Box::new(
            Background::new()
        ));

        Self {
            gameobjects: gameobjects,
            observer: scene_observer
        }
    }
}

impl GameObject for Scene {
    fn behave(&mut self) {
        for gameobject in self.gameobjects.iter_mut() {
            gameobject.behave();
        }

        // check event subscriptions
        for e in self.observer.poll_evt() {
            match e {
                Event::Reset => {
                    agb::println!("score");
                },
                _ => ()
            }
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        for gameobject in self.gameobjects.iter_mut() {
            gameobject.render(frame);
        }
    }
}

impl Subscriber for Scene {
    fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }
}
