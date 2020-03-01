use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    widgets::{Widget, WidgetConfig},
    Assets,
};
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::mint::Vector2;
//use quicksilver::prelude::{Image, Img, Rectangle, Transform, Vector, Window};

///Is used to render text to the screen that the user can click on.
pub struct TextButtonConfig {
    pub text: Image,
    pub location: Rectangle,
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
    fn contains(&self, point: &Vector2<f32>) -> bool {
        point.x >= self.button.location.pos.x
            && point.y >= self.button.location.pos.y
            && point.x <= self.button.location.pos.x + self.button.location.size.x
            && point.y <= self.button.location.pos.y + self.button.location.size.y
    }
    fn is_focusable(&self, _: &Vector2<f32>) -> bool {
        false
    }
    fn render(&self, _: &dyn Assets, gfx: &mut Graphics) {
        gfx.draw_image(&self.button.text, self.button.location)
    }
    fn on_click(&mut self, _location: &Vector2<f32>) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        quicksilver::lifecycle::CursorIcon::Hand
    }
}
