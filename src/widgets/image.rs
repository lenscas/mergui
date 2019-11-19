use super::{RawWidget, Widget};
use crate::Assets;
use quicksilver::prelude::{Img, Rectangle, Transform, Vector, Window};
pub struct Image {
    pub image: String,
    pub location: Rectangle,
}
impl RawWidget<(), Image> for Image {
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
        true
    }
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32) {
        let image = assets.get_image(&self.image);
        window.draw_ex(&self.location, Img(&image), Transform::IDENTITY, z);
    }
    fn on_click(&mut self, _location: &Vector) {
        println!("has clicked")
    }
    fn get_cursor_on_hover(&self) -> quicksilver::input::MouseCursor {
        quicksilver::input::MouseCursor::Hand
    }
    fn on_key_press(
        &mut self,
        key: quicksilver::input::Key,
        state: quicksilver::input::ButtonState,
    ) {
        println!("set key : {:?} to state : {:?}", key, state)
    }
    fn on_typed(&mut self, _char: char) {
        println!("typed char : {:?}", _char)
    }
}
