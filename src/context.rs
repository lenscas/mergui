use crate::widgets::{RawWidget, Widget};
use indexmap::IndexMap;
use quicksilver::{geom::Vector, graphics::Image, input::MouseCursor, lifecycle::Window};
pub struct Response<R> {
    pub channel: R,
    pub id: (u64, u64),
}

pub trait Assets {
    fn get_image(&self, name: &str) -> &Image;
}

pub struct Context<'a> {
    start_z: u32,
    to_display: IndexMap<u64, IndexMap<u64, Box<dyn Widget + 'a>>>,
    widget_with_focus: Option<(u64, u64)>,
    last_layer_id: u64,
    last_view_id: u64,
    mouse_cursor: Vector,
}

impl<'a> Context<'a> {
    pub fn new(cursor: Vector, start_z: u32) -> Self {
        Self {
            to_display: IndexMap::new(),
            widget_with_focus: None,
            last_layer_id: 0,
            last_view_id: 0,
            mouse_cursor: cursor,
            start_z,
        }
    }
    pub fn add_layer(&mut self) -> u64 {
        self.last_layer_id += 1;
        self.to_display.insert(self.last_layer_id, IndexMap::new());
        self.last_layer_id
    }
    pub fn get_focused_widget(&mut self) -> Option<&mut Box<dyn Widget + 'a>> {
        println!("Widget with focus : {:?}", self.widget_with_focus);
        self.widget_with_focus
            .and_then(move |v| self.to_display.get_mut(&v.0).and_then(|x| x.get_mut(&v.1)))
    }
    pub fn event(&mut self, event: &quicksilver::lifecycle::Event, window: &mut Window) {
        use quicksilver::lifecycle::Event::*;
        match event {
            MouseMoved(val) => {
                let mut cursor = MouseCursor::Default;
                for (_, layer) in &self.to_display {
                    for (_, widget) in layer {
                        if widget.contains(&val) {
                            cursor = widget.get_cursor_on_hover();
                        }
                    }
                }
                self.mouse_cursor = *val;
                window.set_cursor(cursor);
            }
            MouseButton(button, state) => {
                if let quicksilver::input::MouseButton::Left = button {
                    if let quicksilver::input::ButtonState::Pressed = state {
                        let mut set_focus = None;
                        'outer: for (layer_id, layer) in self.to_display.iter_mut() {
                            for (widget_key, widget) in layer.iter_mut() {
                                if widget.contains(&self.mouse_cursor) {
                                    widget.on_click(&self.mouse_cursor);
                                    let focus = widget.is_focusable();
                                    println!("has focus : {:?}", focus);
                                    if focus {
                                        set_focus = Some((*layer_id, *widget_key));
                                        break 'outer;
                                    }
                                }
                            }
                        }
                        self.widget_with_focus = set_focus;
                    }
                }
            }
            Key(key, state) => {
                self.get_focused_widget()
                    .map(|focused| focused.on_key_press(*key, *state));
            }
            Typed(typed) => {
                self.get_focused_widget().map(|v| v.on_typed(*typed));
            }
            _ => {}
        }
    }
    pub fn remove_layer(&mut self) {}
    pub fn render<Store: Assets>(&self, assets: &Store, window: &mut Window) {
        let mut z = self.start_z;
        for (_, layer) in &self.to_display {
            for (_, widget) in layer {
                widget.render(assets, window, z);
                z += 1;
            }
        }
    }
    pub fn add_widget<R, W, Res>(&mut self, widget: R, layer_id: u64) -> Result<Response<Res>, ()>
    where
        R: RawWidget<Res, W>,
        W: Widget + 'a,
        Res: Sized,
    {
        match self.to_display.get_mut(&layer_id) {
            Some(layer) => {
                let (widget, res) = widget.to_widget();
                self.last_view_id += 1;
                layer.insert(self.last_view_id, Box::new(widget));
                Ok(Response {
                    channel: res,
                    id: (layer_id, self.last_view_id),
                })
            }
            _ => Err(()),
        }
    }
}
