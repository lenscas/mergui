use crate::widgets::{Widget, WidgetConfig};
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
        self.widget_with_focus
            .and_then(move |v| self.to_display.get_mut(&v.0).and_then(|x| x.get_mut(&v.1)))
    }
    pub fn get_widgets<'b>(
        widgets: &'b IndexMap<u64, IndexMap<u64, Box<dyn Widget + 'a>>>,
    ) -> Vec<((u64, u64), &'b Box<dyn Widget + 'a>)> {
        widgets
            .iter()
            .flat_map(|(layer_id, widgets)| {
                widgets
                    .iter()
                    .map(move |(widget_id, widget)| ((layer_id, widget_id), widget))
            })
            .map(|(id, widget)| ((*id.0, *id.1), widget))
            .collect()
    }
    pub fn get_widgets_mut<'b>(
        widgets: &'b mut IndexMap<u64, IndexMap<u64, Box<dyn Widget + 'a>>>,
    ) -> Vec<((u64, u64), &'b mut Box<dyn Widget + 'a>)> {
        widgets
            .iter_mut()
            .flat_map(|(layer_id, widgets)| {
                widgets
                    .iter_mut()
                    .map(move |(widget_id, widget)| ((layer_id, widget_id), widget))
            })
            .map(|(id, widget)| ((*id.0, *id.1), widget))
            .collect()
    }
    pub fn event(&mut self, event: &quicksilver::lifecycle::Event, window: &mut Window) {
        use quicksilver::input::ButtonState;
        use quicksilver::input::MouseButton;
        use quicksilver::lifecycle::Event::*;
        match event {
            MouseMoved(val) => {
                let widgets = Context::get_widgets_mut(&mut self.to_display);
                window.set_cursor(
                    widgets
                        .iter()
                        .filter(|(_, widget)| widget.contains(&val))
                        .map(|(_, widget)| widget)
                        .collect::<Vec<_>>()
                        .pop()
                        .map(|widget| widget.get_cursor_on_hover())
                        .unwrap_or(MouseCursor::Default),
                );
                self.mouse_cursor = *val;
            }
            MouseButton(MouseButton::Left, ButtonState::Pressed) => {
                let cursor = &self.mouse_cursor;
                let mut widgets = Context::get_widgets_mut(&mut self.to_display);
                let mut maybe_focused_widgets: Vec<_> = widgets
                    .iter_mut()
                    .filter_map(|(id, widget)| {
                        let contains = widget.contains(cursor);
                        let is_focusable = widget.is_focusable();
                        if contains {
                            Some((id, widget, is_focusable))
                        } else {
                            widget.set_focus(false);
                            None
                        }
                    })
                    .collect();
                let cursor = &self.mouse_cursor;
                self.widget_with_focus =
                    maybe_focused_widgets
                        .pop()
                        .map(|(id, widget, is_focusable)| {
                            if is_focusable {
                                widget.set_focus(true)
                            };
                            widget.on_click(cursor);
                            (id.0, id.1)
                        });
                maybe_focused_widgets
                    .iter_mut()
                    .for_each(|(_, widget, is_focusable)| {
                        if *is_focusable {
                            widget.set_focus(false)
                        }
                    });
            }
            MouseButton(_, _) => {
                //we don't handle other mouse buttons/states (yet)
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
        let widgets = Context::get_widgets(&self.to_display);
        widgets.iter().for_each(|(_, widget)| {
            widget.render(assets, window, z);
            z += 1;
        });
    }
    pub fn add_widget<R, W, Res>(&mut self, widget: R, layer_id: u64) -> Result<Response<Res>, ()>
    where
        R: WidgetConfig<Res, W>,
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
