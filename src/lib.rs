#![no_std]

// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::{include_background_gfx, include_aseprite, include_font};
use agb::display::font::Font;
use agb_tracker::{Track, include_xm};

pub mod sprite;
pub mod player;
pub mod gameobject;
pub mod movingstone;
pub mod runningstone;
pub mod observer;
pub mod background;
pub mod scene;
pub mod math;
pub mod music;
pub mod hud;

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
static BGM: Track = include_xm!("sfx/bgm.xm");
static FONT: Font = include_font!("fonts/NESCyrillic.ttf", 16);

// Usage note:
// include_aseprite!(mod sprites, "examples/gfx/chicken.aseprite");
// use sprites::{JUMP, WALK};
// static IDLE: &Sprite = sprites::IDLE.sprite(0);

pub const BALL_SIZE: i32 = 16; // todo: retrieve this value from the sprite
