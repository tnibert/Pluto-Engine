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

use lib::{gameobject::GameObject, music::BackgroundMusic};
use lib::scene::Scene;
use lib::agb_background;


use agb::{
    display::tiled::VRAM_MANAGER,
    interrupt::{add_interrupt_handler, Interrupt},
    sound::mixer::Frequency
};
use critical_section::CriticalSection;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    agb::println!("Start main");
    // Get the graphics manager, responsible for all the graphics
    let mut gfx = gba.graphics.get();

    // vblank interrupt handler
    unsafe {
        let _ = add_interrupt_handler(Interrupt::VBlank, |_: CriticalSection| {
            agb::println!("Woah there! There's been a vblank!");
        });
    };

    VRAM_MANAGER.set_background_palettes(agb_background::PALETTES);

    // initialize the game
    let mut game = Scene::new();

    // setup the background music
    // todo: encapsulate this into the Scene
    let mut mixer = gba.mixer.mixer(Frequency::Hz32768);
    let mut bgm = BackgroundMusic::new(&mut mixer);

    loop {
        // update the scene
        game.behave();
        bgm.behave();

        // render the scene
        let mut frame = gfx.frame();
        game.render(&mut frame);
        bgm.render(&mut frame);
        frame.commit();
    }
}
