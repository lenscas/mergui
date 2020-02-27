use super::{Widget, WidgetConfig};
use crate::{
    channels::clickable::{BasicClickable as Clickable, ClickSetter as Channel},
    core::{
        image_button::{ImageButton, ImageButtonConfig},
        text_button::{TextButton, TextButtonConfig},
    },
    Assets,
};
use quicksilver::geom::Rectangle;
use quicksilver::graphics::Color;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::mint::Vector2;

pub struct ButtonConfig {
    ///The text that will be rendered
    pub text: Image,
    ///the name of the image for the background.
    pub background: String,
    ///where the text needs to be rendered
    pub text_location: Rectangle,
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
        self.text_location.pos += self.background_location.pos;
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
                    location: self.text_location,
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
    fn contains(&self, point: &Vector2<f32>) -> bool {
        let contains = self.background.contains(point) || self.text.contains(point);

        contains
    }
    fn is_focusable(&self, _: &Vector2<f32>) -> bool {
        false
    }
    fn set_hover(&mut self, _: &Vector2<f32>, hover: bool) {
        self.background.is_hovering = hover;
    }
    fn render(&self, assets: &dyn Assets, gfx: &mut Graphics, z: u32) {
        self.background.render(assets, gfx, z);
        self.text.render(assets, gfx, z + 1);
    }
    fn on_click(&mut self, _: &Vector2<f32>) {
        self.channel.clicked();
    }
    fn get_cursor_on_hover(&self, _: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        quicksilver::lifecycle::CursorIcon::Hand
    }
}
