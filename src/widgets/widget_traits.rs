use quicksilver::graphics::Graphics;

use quicksilver::{Window, geom::Vector};

///Turns a simple configuration into a real widget that can be drawn and interacted with.
pub trait WidgetConfig<R: Sized, W: Widget> {
    fn to_widget(self) -> (W, R);
}
///This is the real widget. It isn't meant to interact directly with except when creating other widgets that exist of multiple smaller ones
pub trait Widget {
    fn contains(&self, pos: &Vector) -> bool;
    fn is_focusable(&self, pos: &Vector) -> bool;
    fn render(&mut self, gfx: &mut Graphics, window: &Window) -> quicksilver::Result<()>;
    fn get_cursor_on_hover(&self, _: &Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Default
    }
    fn set_focus(&mut self, _: &Vector, _: bool) {}
    fn set_hover(&mut self, _: &Vector, _: bool) {}
    fn on_click(&mut self, _location: &Vector) {}
    fn on_key_press(&mut self, _key: quicksilver::blinds::event::Key, _state: bool) {}
    fn on_typed(&mut self, _char: char) {}
}
