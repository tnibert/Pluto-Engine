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

use agb::{
    include_aseprite,
    include_background_gfx,
    display::{
        object::{Graphics, Tag, Object},
        tiled::{
            RegularBackgroundSize, TiledMap,
        },
        Priority,
    },
};
use agb::interrupt::{Interrupt, add_interrupt_handler};
use agb::input::{Button, ButtonController};
use critical_section::CriticalSection;

// Import the sprites in to this static. This holds the sprite 
// and palette data in a way that is manageable by agb.
static GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
//include_aseprite!(mod sprites, "examples/gfx/chicken.aseprite");
//use sprites::{JUMP, WALK};
//static IDLE: &Sprite = sprites::IDLE.sprite(0);

include_background_gfx!(map_tiles, tiles => "gfx/water_tiles.png");

// We define some easy ways of referencing the sprites
/*const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");*/
static BALL: &Tag = GRAPHICS.tags().get("Ball");
const BALL_SIZE: i32 = 16;

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
    object: Object <'a>,
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
    let oam = gba.display.object.get_managed();

    // vblank interrupt handler
   unsafe {
        let _ = add_interrupt_handler(Interrupt::VBlank, |_: CriticalSection| {
            agb::println!("Woah there! There's been a vblank!");
        });
    };

    let mut input = ButtonController::new();
    let (gfx, mut vram) = gba.display.video.tiled0();

    // Create an object with the ball sprite
    let mut ball = Sprite {
        x: agb::display::WIDTH / 2 - BALL_SIZE/2,
        y: agb::display::HEIGHT / 2 - BALL_SIZE/2,
        velocity: 1,
        object: oam.object_sprite(BALL.sprite(0)),
    };

    let mut ball2 = Sprite {
        x: agb::display::WIDTH - BALL_SIZE,
        y: agb::display::HEIGHT - BALL_SIZE,
        velocity: 1,
        object: oam.object_sprite(BALL.sprite(0)),
    };

    let tileset = &map_tiles::tiles.tiles;

    vram.set_background_palettes(map_tiles::PALETTES);

    let mut bg = gfx.background(Priority::P0, RegularBackgroundSize::Background32x32, tileset.format());

    for y in 0..20u16 {
        for x in 0..30u16 {
            bg.set_tile(
                &mut vram,
                (x, y),
                tileset,
                map_tiles::tiles.tile_settings[0],
            );
        }
    }
    bg.commit(&mut vram);
    bg.set_visible(true);

    // game loop
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

        if ball2.x+BALL_SIZE <= 0 {
            ball2.x = agb::display::WIDTH;
        } else {
            ball2.update_pos(Direction::LEFT);
        }

        // Set the position of the sprite to match our new calculated position
        ball.object.set_x(ball.x as u16).set_y(ball.y as u16).show();
        ball2.object.set_x(ball2.x as u16).set_y(ball2.y as u16).show();
    
        // Wait for vblank, then commit the objects to the screen
        // todo: don't busy wait for vblank, use interrupt
        agb::display::busy_wait_for_vblank();
        oam.commit();
    }
}
