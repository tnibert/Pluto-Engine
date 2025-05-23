// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

//use agb::{display, syscall};
use agb::{include_aseprite,
    display::object::{Graphics, Tag, Object}
};
use agb::interrupt::{Interrupt, add_interrupt_handler};
use agb::input::{Button, ButtonController};
use critical_section::CriticalSection;

// Import the sprites in to this static. This holds the sprite 
// and palette data in a way that is manageable by agb.
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// We define some easy ways of referencing the sprites
/*const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");*/
static BALL: &Tag = GRAPHICS.tags().get("Ball");

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Sprite <'a> {
    x: i32,
    y: i32,
    velocity: i32,
    object: Object <'a>
}

impl Sprite <'_> {
    pub fn update_pos(&mut self, dir: Direction) {
        match dir {
            Direction::LEFT => self.x -= self.velocity,
            Direction::RIGHT => self.x += self.velocity,
            Direction::UP => self.y -= self.velocity,
            Direction::DOWN => self.y += self.velocity
        }
    }
}

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {    
    // Get the OAM manager
    let object = gba.display.object.get_managed();

    // vblank interrupt handler
    unsafe {
        let _ = add_interrupt_handler(Interrupt::VBlank, |_: CriticalSection| {
            agb::println!("Woah there! There's been a vblank!");
        });
    };

    let mut input = ButtonController::new();

    // Create an object with the ball sprite
    let mut ball = Sprite {
        x: agb::display::WIDTH / 2 - 8,     // todo: make 16 a constant
        y: agb::display::HEIGHT / 2 - 8,
        velocity: 1,
        object: object.object_sprite(BALL.sprite(0))
    };
    
    loop {
        // handle input to move ball
        input.update();
        if input.is_pressed(Button::UP) && ball.y > 0 {
            ball.update_pos(Direction::UP);
        }
        if input.is_pressed(Button::DOWN) && ball.y < agb::display::HEIGHT - 16 {
            ball.update_pos(Direction::DOWN);
        }
        if input.is_pressed(Button::LEFT) && ball.x > 0 {
            ball.update_pos(Direction::LEFT);
        }
        if input.is_pressed(Button::RIGHT) && ball.x < agb::display::WIDTH - 16 {
            ball.update_pos(Direction::RIGHT);
        }

        // Set the position of the sprite to match our new calculated position
        ball.object.set_x(ball.x as u16).set_y(ball.y as u16).show();
    
        // Wait for vblank, then commit the objects to the screen
        // todo: don't busy wait for vblank, use interrupt
        agb::display::busy_wait_for_vblank();
        object.commit();
    }
}
