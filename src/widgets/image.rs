use super::{Widget, WidgetConfig};
use crate::Assets;
use quicksilver::prelude::{Img, Rectangle, Transform, Vector, Window};
pub struct Image {
    pub image: String,
    pub location: Rectangle,
}
impl WidgetConfig<(), Image> for Image {
    fn to_widget(self) -> (Image, ()) {
        (self, ())
    }
}
impl Widget for Image {
    fn contains(&self, point: &Vector) -> bool {
        point.x >= self.location.pos.x
            && point.y >= self.location.pos.y
            && point.x <= self.location.pos.x + self.location.size.x
            && point.y <= self.location.pos.y + self.location.size.y
    }
    fn is_focusable(&self) -> bool {
        false
    }
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32) {
        let image = assets.get_image(&self.image);
        window.draw_ex(&self.location, Img(&image), Transform::IDENTITY, z);
    }
}
