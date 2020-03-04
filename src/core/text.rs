use crate::widgets::{Widget, WidgetConfig};
use crate::FontStyle;
use quicksilver::graphics::Graphics;
use quicksilver::mint::Vector2;
//use quicksilver::prelude::{Image, Img, Rectangle, Transform, Vector, Window};

///Is used to render text to the screen
pub struct Text {
    pub text: String,
    pub font_style: FontStyle,
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
    fn render(&mut self, gfx: &mut Graphics) {
        gfx.draw_text(
            &mut self.font_style.font.0.borrow_mut(),
            &self.text,
            self.font_style.size,
            self.font_style.max_width,
            self.font_style.color,
            self.font_style.location,
        )
    }
}
