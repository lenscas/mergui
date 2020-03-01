use crate::{
    widgets::{Widget, WidgetConfig},
    Assets,
};
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::mint::Vector2;
//use quicksilver::prelude::{Image, Img, Rectangle, Transform, Vector, Window};

///Is used to render text to the screen
pub struct Text {
    pub text: Image,
    pub location: Rectangle,
}

impl WidgetConfig<(), Text> for Text {
    fn to_widget(self) -> (Text, ()) {
        (self, ())
    }
}

impl Widget for Text {
    fn contains(&self, _: &Vector2<f32>) -> bool {
        false
    }
    fn is_focusable(&self, _: &Vector2<f32>) -> bool {
        false
    }
    fn render(&self, _: &dyn Assets, gfx: &mut Graphics) {
        gfx.draw_image(&self.text, self.location)
    }
}
