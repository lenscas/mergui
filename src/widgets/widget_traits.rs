use quicksilver::{geom::Vector, graphics::Graphics, Window};

///Turns a simple configuration into a real widget that can be drawn and interacted with.
pub trait WidgetConfig<R: Sized, W: Widget> {
    ///Turn the config into an actual widget
    fn to_widget(self) -> (W, R);
    ///Use the config to edit a widget.
    ///This can be more efficient than calling Self::to_widget if you want to edit a widget
    ///However there are also cases where it just calls to_widget behind the scenes
    fn edit_widget(self, widget: W, return_channel: R) -> (W, R);
}

///This is the real widget. It isn't meant to interact directly with except when creating other widgets that exist of multiple smaller ones
pub trait Widget {
    fn contains(&self, pos: Vector) -> bool;
    fn is_focusable(&self, pos: Vector) -> bool;
    fn render(&mut self, gfx: &mut Graphics, window: &Window) -> quicksilver::Result<()>;
    fn get_cursor_on_hover(&self, _: Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Default
    }
    fn set_focus(&mut self, _: Vector, _: bool) {}
    fn set_hover(&mut self, _: Vector, _: bool) {}
    fn on_click(&mut self, _location: Vector) {}
    fn on_key_press(&mut self, _key: quicksilver::input::Key, _state: bool) {}
    fn on_typed(&mut self, _char: char) {}
}
