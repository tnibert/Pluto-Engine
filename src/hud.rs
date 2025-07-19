extern crate alloc;

use crate::observer::{Event, Listener, Subscriber};
use crate::gameobject::GameObject;
use crate::FONT;
use agb::fixnum::vec2;
use alloc::format;
use agb::{
    display::{
        Palette16, Rgb15,
        GraphicsFrame,
        font::{
            Layout, AlignmentKind, ObjectTextRenderer
        },
        object::Size
    },
};
use alloc::rc::Rc;
use alloc::vec::Vec;

const MAX_GROUP_WIDTH: i32 = 32;
const MAX_LINE_LENGTH: i32 = 20;
const SCORE_X: i32 = agb::display::WIDTH / 2;
const SCORE_Y: i32 = 0;

static PALETTE: &Palette16 = const {
    let mut palette = [Rgb15::BLACK; 16];
    palette[1] = Rgb15::WHITE;
    palette[2] = Rgb15(0x10_7C);
    &Palette16::new(palette)
};

pub struct HUD{
    renderer: ObjectTextRenderer,
    score: i32,
    observer: Rc<Listener>
}

impl HUD{
    pub fn new() -> HUD {
        // todo: better use of resource to make HUD a background layer - establish feasibility
        let text_renderer = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);

        Self {
            renderer: text_renderer,
            score: 0,
            observer: Rc::new(Listener::new())
        }
    }
}

impl GameObject for HUD {
    fn behave(&mut self) {
        for e in self.observer.poll_evt() {
            match e {
                Event::Reset => {
                    self.score += 5;
                },
                _ => ()
            }
        }
    }

    fn render(&mut self, frame: &mut GraphicsFrame) {
        let layout = Layout::new(
            format!("{:5}", self.score).as_str(),
            &FONT,
            AlignmentKind::Centre,
            MAX_GROUP_WIDTH,
            MAX_LINE_LENGTH,
        );
        let objects: Vec<_> = layout.map(|x| self.renderer.show(&x, vec2(SCORE_X, SCORE_Y))).collect();
        for object in objects.iter() {
            object.show(frame);
        }
    }
}

impl Subscriber for HUD {
    fn observer(&self) -> Rc<Listener> {
        return self.observer.clone()
    }
}
