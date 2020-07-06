use crate::widgets::{Widget, WidgetConfig};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    Result, Window,
};
//use quicksilver::prelude::{Img, Rectangle, Transform, Vector, Window};
///A simple unfocusable, uninteractable image.
#[derive(Clone)]
pub struct Image {
    ///name of the image that needs to be rendered
    pub image: quicksilver::graphics::Image,
    ///location and size that will be used to render the image
    pub location: Rectangle,
}
impl WidgetConfig<(), Image> for Image {
    fn to_widget(self) -> (Image, ()) {
        (self, ())
    }
}
impl Widget for Image {
    fn contains(&self, _: Vector) -> bool {
        false
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        gfx.draw_image(&self.image, self.location);
        Ok(())
    }
}
