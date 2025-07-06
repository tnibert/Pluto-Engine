#![no_std]

// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{include_background_gfx, include_aseprite};

pub mod sprite;
pub mod player;
pub mod gameobject;
pub mod movingstone;
pub mod runningstone;
pub mod observer;
pub mod background;
pub mod scene;
pub mod math;

// load assets into binary
include_background_gfx!(
    pub mod agb_background,
    TILES => deduplicate "gfx/beach-background.aseprite"
);
include_aseprite!(
    pub mod agb_sprites,
    "gfx/sprites.aseprite"
);
include_aseprite!(
    pub mod agb_crab,
    "gfx/crab-small.aseprite"
);


// Usage note:
// include_aseprite!(mod sprites, "examples/gfx/chicken.aseprite");
// use sprites::{JUMP, WALK};
// static IDLE: &Sprite = sprites::IDLE.sprite(0);

pub const BALL_SIZE: i32 = 16; // todo: retrieve this value from the sprite
