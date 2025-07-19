extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::rc::Rc;
use agb::{display::{object::Object, GraphicsFrame}, fixnum::{vec2, Rect}};

use crate::{
    agb_crab, agb_sprites, background::Background, gameobject::GameObject, hud::HUD, math::random_constrained_positive, movingstone::MovingStone, observer::{Event, Listener, Publisher, Subscriber}, player::Player, runningstone::RunningStone, BALL_SIZE
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
        let crab = Object::new(agb_crab::SURPRISED.sprite(0));

        // Create game objects
        let mut gameobjects: Vec<Box<dyn GameObject>> = Vec::new();
        let mut player = Box::new(Player::new(crab));
        let mut running_stone = Box::new(RunningStone::new(20, 0, ball));
        let hud = Box::new(HUD::new());
        let bg = Box::new(Background::new());

        // establish communication pathways
        // create moving stones
        for i in 1..9 {
            let img: Object;
            if i & 1 == 0 {
                img = paddle_mid.clone();
            } else {
                img = paddle_end.clone();
            }
            let mut moving_stone = Box::new(MovingStone::new(random_constrained_positive(agb::display::WIDTH - BALL_SIZE), BALL_SIZE * i, img));
            moving_stone.register_subscription(player.observer(), Event::Position(Rect::new(vec2(0, 0), vec2(0, 0))));
            gameobjects.push(moving_stone);
        }

        // running stone listens for position from player
        player.register_subscription(running_stone.observer(), Event::Position(Rect::new(vec2(0, 0), vec2(0, 0))));
        // scene listens for reset from running stone
        running_stone.register_subscription(scene_observer.clone(), Event::Reset);
        // todo: HUD listens for reset from running stone
        running_stone.register_subscription(hud.observer(), Event::Reset);

        // add gameobjects to the scene graph
        gameobjects.push(player);
        gameobjects.push(running_stone);
        gameobjects.push(bg);
        gameobjects.push(hud);

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
