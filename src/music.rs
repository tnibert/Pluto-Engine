extern crate alloc;

use agb::display::GraphicsFrame;
use agb::sound::mixer::Mixer;
use agb_tracker::Tracker;

use crate::gameobject::GameObject;
use crate::BGM;

pub struct BackgroundMusic <'a>{
    mixer: &'a mut Mixer <'a>,
    tracker: Tracker
}

impl <'a> BackgroundMusic <'a>{
    pub fn new(m: &'a mut Mixer <'a>) -> BackgroundMusic <'a> {
        Self {
            mixer: m,
            tracker: Tracker::new(&BGM)
        }
    }
}

impl <'a> GameObject for BackgroundMusic <'a> {
    fn behave(&mut self) {
        self.tracker.step(self.mixer);
    }

    fn render(&mut self, _frame: &mut GraphicsFrame) {
        self.mixer.frame();
    }
}
