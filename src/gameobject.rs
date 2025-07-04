use agb::display::GraphicsFrame;

pub trait GameObject {
    fn behave(&mut self);
    fn render(&mut self, frame: &mut GraphicsFrame);
}
