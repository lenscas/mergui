use crate::widgets::{Widget, WidgetConfig};
use crate::FontStyle;
use quicksilver::graphics::Graphics;
use quicksilver::{geom::Vector, Result, Window};

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
    fn contains(&self, _: Vector) -> bool {
        false
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        self.font_style.draw(gfx, &self.text)?;
        Ok(())
    }
}
