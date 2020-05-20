use super::{Widget, WidgetConfig};
use crate::FontStyle;
use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    core::{
        image_button::{ImageButton, ImageButtonConfig},
        text_button::{TextButton, TextButtonConfig},
    },
};
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::Color;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::{Result, Window};

pub struct ButtonConfig {
    ///The text that will be rendered
    pub text: String,
    ///The style that will be used for the text
    pub font_style: FontStyle,
    ///the name of the image for the background.
    pub background: Image,
    ///where the background needs to be rendered
    pub background_location: Rectangle,
    ///optionally, the color that the background needs to blend with
    pub blend_color: Option<Color>,
    ///optionally, the color the background needs to blend with if the user hovers over it
    pub hover_color: Option<Color>,
}

pub struct Button {
    pub text: TextButton,
    pub background: ImageButton,
    pub channel: Channel,
}

impl WidgetConfig<Clickable, Button> for ButtonConfig {
    fn to_widget(mut self) -> (Button, Clickable) {
        let (res, channel) = Clickable::new();
        self.font_style.location += self.background_location.pos;
        (
            Button {
                background: ImageButtonConfig {
                    image: self.background,
                    location: self.background_location,
                    color: self.blend_color,
                    hover_color: self.hover_color,
                }
                .to_widget()
                .0,
                text: TextButtonConfig {
                    text: self.text,
                    font_style: self.font_style,
                }
                .to_widget()
                .0,
                channel,
            },
            res,
        )
    }
}

impl Widget for Button {
    fn contains(&self, point: &Vector) -> bool {
        self.background.contains(point) || self.text.contains(point)
    }
    fn is_focusable(&self, _: &Vector) -> bool {
        false
    }
    fn set_hover(&mut self, _: &Vector, hover: bool) {
        self.background.is_hovering = hover;
    }
    fn render(&mut self, gfx: &mut Graphics, w: &Window) -> Result<()> {
        self.background.render(gfx, w)?;
        self.text.render(gfx, w)?;
        Ok(())
    }
    fn on_click(&mut self, _: &Vector) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: &Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Hand
    }
}
