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

use lib::gameobject::GameObject;
use lib::player::Player;
use lib::movingstone::MovingStone;
use lib::runningstone::RunningStone;
use lib::background::Background;
use lib::agb_background;
use lib::BALL_SIZE;

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;

use agb::{
    display::{
        object::Object,
        tiled::VRAM_MANAGER
    },
    include_aseprite,
    interrupt::{add_interrupt_handler, Interrupt}
};
use critical_section::CriticalSection;

// Import the sprites in to this static. This holds the sprite 
// and palette data in a way that is manageable by agb.
include_aseprite!(
    mod agb_sprites,
    "gfx/sprites.aseprite"
);

//include_aseprite!(mod sprites, "examples/gfx/chicken.aseprite");
//use sprites::{JUMP, WALK};
//static IDLE: &Sprite = sprites::IDLE.sprite(0);

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    agb::println!("Start main");
    // Get the graphics manager, responsible for all the graphics
    let mut gfx = gba.graphics.get();

    let ball = Object::new(agb_sprites::BALL.sprite(0));
    let paddle_mid = Object::new(agb_sprites::PADDLE_MID.sprite(0));
    let paddle_end = Object::new(agb_sprites::PADDLE_END.sprite(0));

    // vblank interrupt handler
    unsafe {
        let _ = add_interrupt_handler(Interrupt::VBlank, |_: CriticalSection| {
            agb::println!("Woah there! There's been a vblank!");
        });
    };

    VRAM_MANAGER.set_background_palettes(agb_background::PALETTES);

    // Create game objects
    let mut gameobjects: Vec<Box<dyn GameObject>> = Vec::new();
    let mut player = Box::new(Player::new(ball));
    let mut moving_stone = Box::new(MovingStone::new(agb::display::WIDTH/2, agb::display::HEIGHT - BALL_SIZE, paddle_mid));

    // set up communication pathways
    // player listens for reset from moving stone
    let mut evts = Vec::new();
    evts.push(String::from("reset"));
    // todo: unify subscribe() behind a trait, with a common signature between it and Observable
    moving_stone.subscribe(player.observer(), evts);

    // running stone listens for position from player
    let mut evts = Vec::new();
    evts.push(String::from("position"));
    let running_stone = Box::new(RunningStone::new(20, 20, paddle_end));
    player.subscribe(running_stone.observer(), evts);

    gameobjects.push(player);
    gameobjects.push(moving_stone);
    gameobjects.push(running_stone);
    gameobjects.push(Box::new(
        Background::new()
    ));

    // game loop
    loop {
        let mut frame = gfx.frame();
        for gameobject in gameobjects.iter_mut() {
            gameobject.behave();
            gameobject.render(&mut frame);
        }
        frame.commit();
    }
}
