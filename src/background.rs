use crate::gameobject::GameObject;
use crate::agb_background;
use agb::{
    display::{
        GraphicsFrame,
        tiled::{
            RegularBackground, RegularBackgroundSize, TileFormat
        }, Priority,
        tiled::VRAM_MANAGER
    },
};

pub struct Background{
    bg: RegularBackground
}

impl Background{
    pub fn new() -> Background {
        // create background
        VRAM_MANAGER.set_background_palettes(agb_background::PALETTES);
        let mut bg = RegularBackground::new(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp
        );

        bg.fill_with(&agb_background::TILES);

        Self {
            bg: bg
        }
    }
}

impl GameObject for Background {
    fn behave(&mut self) {}

    fn render(&mut self, frame: &mut GraphicsFrame) {
        self.bg.show(frame);
    }
}
