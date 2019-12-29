use crate::{
    widgets::{Widget, WidgetConfig},
    Assets,
};
use quicksilver::prelude::{Image, Img, Rectangle, Transform, Vector, Window};

///Is used to render text to the screen
pub struct Text {
    pub text: Image,
    pub location: Rectangle,
}

impl WidgetConfig<(), Text> for Text {
    fn to_widget(self) -> (Text, ()) {
        (self, ())
    }
}

impl Widget for Text {
    fn contains(&self, _: &Vector) -> bool {
        false
    }
    fn is_focusable(&self, _: &Vector) -> bool {
        false
    }
    fn render(&self, _: &dyn Assets, window: &mut Window, z: u32) {
        window.draw_ex(&self.location, Img(&self.text), Transform::IDENTITY, z);
    }
}
