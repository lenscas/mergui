use crate::FontStyle;
use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    widgets::{Widget, WidgetConfig},
};
use quicksilver::graphics::Graphics;
use quicksilver::{geom::Vector, Result, Window};
//use quicksilver::prelude::{Image, Img, Rectangle, Transform, Vector, Window};

///Is used to render text to the screen that the user can click on.
pub struct TextButtonConfig {
    pub text: String,
    pub font_style: FontStyle,
}

pub struct TextButton {
    pub button: TextButtonConfig,
    pub channel: Channel,
}

impl WidgetConfig<Clickable, TextButton> for TextButtonConfig {
    fn to_widget(self) -> (TextButton, Clickable) {
        let (res, channel) = Clickable::new();
        (
            TextButton {
                button: self,
                channel,
            },
            res,
        )
    }
}

impl Widget for TextButton {
    fn contains(&self, _: Vector) -> bool {
        false
        /*point.x >= self.button.location.pos.x
        && point.y >= self.button.location.pos.y
        && point.x <= self.button.location.pos.x + self.button.location.size.x
        && point.y <= self.button.location.pos.y + self.button.location.size.y*/
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        self.button.font_style.draw(gfx, &self.button.text)?;
        Ok(())
    }
    fn on_click(&mut self, _location: Vector) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Hand
    }
}
