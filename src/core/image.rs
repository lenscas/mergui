use crate::{
    widgets::{Widget, WidgetConfig},
    Assets,
};
use quicksilver::prelude::{Img, Rectangle, Transform, Vector, Window};
///A simple unfocusable, uninteractable image.
pub struct Image {
    ///name of the image that needs to be rendered
    pub image: String,
    ///location and size that will be used to render the image
    pub location: Rectangle,
}
impl WidgetConfig<(), Image> for Image {
    fn to_widget(self) -> (Image, ()) {
        (self, ())
    }
}
impl Widget for Image {
    fn contains(&self, _: &Vector) -> bool {
        false
    }
    fn is_focusable(&self, _: &Vector) -> bool {
        false
    }
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32) {
        let image = assets.get_image(&self.image);
        window.draw_ex(&self.location, Img(&image), Transform::IDENTITY, z);
    }
}
