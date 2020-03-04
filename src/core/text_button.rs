use crate::FontStyle;
use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    widgets::{Widget, WidgetConfig},
};
use quicksilver::graphics::Graphics;
use quicksilver::mint::Vector2;
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
    fn contains(&self, _: &Vector2<f32>) -> bool {
        false
        /*point.x >= self.button.location.pos.x
        && point.y >= self.button.location.pos.y
        && point.x <= self.button.location.pos.x + self.button.location.size.x
        && point.y <= self.button.location.pos.y + self.button.location.size.y*/
    }
    fn is_focusable(&self, _: &Vector2<f32>) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics) {
        gfx.draw_text(
            &mut self.button.font_style.font.0.borrow_mut(),
            &self.button.text,
            self.button.font_style.size,
            self.button.font_style.max_width,
            self.button.font_style.color,
            self.button.font_style.location,
        )
    }
    fn on_click(&mut self, _location: &Vector2<f32>) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        quicksilver::lifecycle::CursorIcon::Hand
    }
}
