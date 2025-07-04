#![no_std]

// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::include_background_gfx;

pub mod sprite;
pub mod player;
pub mod gameobject;
pub mod movingstone;
pub mod observer;
pub mod background;
include_background_gfx!(pub mod agb_background, TILES => deduplicate "gfx/beach-background.aseprite");


pub const BALL_SIZE: i32 = 16; // todo: retrieve this value from the sprite
