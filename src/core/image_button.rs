use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    widgets::{Widget, WidgetConfig},
};
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::Color;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::{Result, Window};

//use quicksilver::prelude::{Blended, Color, Img, Rectangle, Transform, Vector, Window};

///Similar to the Image widget, except it notifies back when the user clicked on it.
pub struct ImageButtonConfig {
    pub image: Image,
    pub color: Option<Color>,
    pub hover_color: Option<Color>,
    pub location: Rectangle,
}

pub struct ImageButton {
    pub button: ImageButtonConfig,
    pub channel: Channel,
    pub is_hovering: bool,
}

impl WidgetConfig<Clickable, ImageButton> for ImageButtonConfig {
    fn to_widget(self) -> (ImageButton, Clickable) {
        let (res, channel) = Clickable::new();
        (
            ImageButton {
                button: self,
                channel,
                is_hovering: false,
            },
            res,
        )
    }
}

impl Widget for ImageButton {
    fn contains(&self, point: Vector) -> bool {
        point.x >= self.button.location.pos.x
            && point.y >= self.button.location.pos.y
            && point.x <= self.button.location.pos.x + self.button.location.size.x
            && point.y <= self.button.location.pos.y + self.button.location.size.y
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        match (self.button.color, self.button.hover_color, self.is_hovering) {
            (Some(color), _, false) | (Some(color), None, true) => {
                gfx.draw_image_tinted(&self.button.image, self.button.location, color)
            }
            (_, Some(color2), true) => {
                gfx.draw_image_tinted(&self.button.image, self.button.location, color2)
            }
            (None, None, _) | (None, Some(_), false) => {
                gfx.draw_image(&self.button.image, self.button.location)
            }
        };
        Ok(())
    }
    fn on_click(&mut self, _location: Vector) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Hand
    }
}
