use crate::Assets;
use quicksilver::graphics::Graphics;

use quicksilver::mint::Vector2;

///Turns a simple configuration into a real widget that can be drawn and interacted with.
pub trait WidgetConfig<R: Sized, W: Widget> {
    fn to_widget(self) -> (W, R);
}
///This is the real widget. It isn't meant to interact directly with except when creating other widgets that exist of multiple smaller ones
pub trait Widget {
    fn contains(&self, pos: &Vector2<f32>) -> bool;
    fn is_focusable(&self, pos: &Vector2<f32>) -> bool;
    fn render(&self, assets: &dyn Assets, gfx: &mut Graphics);
    fn get_cursor_on_hover(&self, _: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        quicksilver::lifecycle::CursorIcon::Default
    }
    fn set_focus(&mut self, _: &Vector2<f32>, _: bool) {}
    fn set_hover(&mut self, _: &Vector2<f32>, _: bool) {}
    fn on_click(&mut self, _location: &Vector2<f32>) {}
    fn on_key_press(&mut self, _key: quicksilver::blinds::event::Key, _state: bool) {}
    fn on_typed(&mut self, _char: char) {}
}
