use crate::widgets::{Widget, WidgetConfig};
use indexmap::IndexMap;
use quicksilver::{geom::Vector, graphics::Image, input::MouseCursor, lifecycle::Window};

/// This is the struct that gets returned when a widget is added to the context.
pub struct Response<R> {
    ///This is used to comunicate with the widget
    pub channel: R,
    ///This is the id of the widget, allowing you to remove it from the context
    pub id: (u64, u64),
}
struct Layer<'a> {
    is_active: bool,
    widgets: IndexMap<u64, Box<dyn Widget + 'a>>,
    current_id: u64,
}
impl<'a> Default for Layer<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> Layer<'a> {
    pub fn new() -> Self {
        Self {
            is_active: true,
            widgets: Default::default(),
            current_id: 0,
        }
    }
    pub fn get_mut(&mut self, index: &u64) -> Option<&mut Box<dyn Widget + 'a>> {
        self.widgets.get_mut(index)
    }
    pub fn remove(&mut self, index: &u64) {
        self.widgets.remove(index);
    }
    pub fn insert(&mut self, widget: Box<dyn Widget + 'a>) -> u64 {
        self.current_id += 1;
        self.widgets.insert(self.current_id, widget);
        self.current_id
    }
}
///Used by widgets to get the image they need to render.
pub trait Assets {
    fn get_image(&self, name: &str) -> &Image;
}
///This manages the GUI. It contains every widget that needs to be drawn and makes sure they are updated properly
pub struct Context<'a> {
    start_z: u32,
    to_display: IndexMap<u64, Layer<'a>>,
    widget_with_focus: Option<(u64, u64)>,
    last_layer_id: u64,
    mouse_cursor: Vector,
}

impl<'a> Context<'a> {
    ///Due to bug [434](https://github.com/ryanisaacg/quicksilver/issues/434) in quicksilver, the draw order of draw calls without a "z" value is unspecified
    ///
    ///To get arround this, the context will draw every widget with its own Z value.
    ///
    ///It draws the first widget with start_z and increases it by one for every widget, resseting this back to start_z every frame
    pub fn new(cursor: Vector, start_z: u32) -> Self {
        Self {
            to_display: IndexMap::new(),
            widget_with_focus: None,
            last_layer_id: 0,
            mouse_cursor: cursor,
            start_z,
        }
    }
    ///Adds a layer that can hold multiple widgets.
    ///Usefull to group widgets together that need to be removed at the same time
    pub fn add_layer(&mut self) -> u64 {
        self.last_layer_id += 1;
        self.to_display
            .insert(self.last_layer_id, Default::default());
        self.last_layer_id
    }
    ///Removes a layer and every widget inside of it.
    ///Usefull to remove multiple widgets at the same time
    pub fn remove_layer(&mut self, layer_id: u64) {
        self.to_display.remove(&layer_id);
    }

    fn get_focused_widget(&mut self) -> Option<&mut Box<dyn Widget + 'a>> {
        self.widget_with_focus
            .and_then(move |v| self.to_display.get_mut(&v.0).and_then(|x| x.get_mut(&v.1)))
    }
    fn get_widgets<'b>(
        widgets: &'b IndexMap<u64, Layer<'a>>,
    ) -> Vec<((u64, u64), &'b Box<dyn Widget + 'a>)> {
        widgets
            .iter()
            .filter(|(_, layer)| layer.is_active)
            .flat_map(|(layer_id, layer)| {
                layer
                    .widgets
                    .iter()
                    .map(move |(widget_id, widget)| ((layer_id, widget_id), widget))
            })
            .map(|(id, widget)| ((*id.0, *id.1), widget))
            .collect()
    }
    fn get_widgets_mut<'b>(
        widgets: &'b mut IndexMap<u64, Layer<'a>>,
    ) -> Vec<((u64, u64), &'b mut Box<dyn Widget + 'a>)> {
        widgets
            .iter_mut()
            .filter(|(_, layer)| layer.is_active)
            .flat_map(|(layer_id, layer)| {
                layer
                    .widgets
                    .iter_mut()
                    .map(move |(widget_id, widget)| ((layer_id, widget_id), widget))
            })
            .map(|(id, widget)| ((*id.0, *id.1), widget))
            .collect()
    }
    ///Call this in the event function of the state to update every widget.
    pub fn event(&mut self, event: &quicksilver::lifecycle::Event, window: &mut Window) {
        use quicksilver::input::ButtonState;
        use quicksilver::input::MouseButton;
        use quicksilver::lifecycle::Event::*;
        match event {
            MouseMoved(val) => {
                let mut widgets = Context::get_widgets_mut(&mut self.to_display);
                let mut widgets = widgets
                    .iter_mut()
                    .filter_map(|(_, widget)| {
                        let does_hover = widget.contains(&val);
                        if does_hover {
                            Some(widget)
                        } else {
                            widget.set_hover(false);
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let cursor = widgets
                    .pop()
                    .map(|widget| {
                        widget.set_hover(true);
                        widget.get_cursor_on_hover()
                    })
                    .unwrap_or(MouseCursor::Default);
                widgets.iter_mut().for_each(|v| v.set_hover(false));

                window.set_cursor(cursor);
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
    ///Set a layer to active or inactive.
    ///Layers that are inactive won't be rendered or receive updates.
    pub fn set_layer_state(&mut self, layer_id: u64, state: bool) {
        self.to_display
            .get_mut(&layer_id)
            .map(|v| v.is_active = state);
    }
    ///Removes a widget, meaning it won't be updated or drawn ever again
    pub fn remove_widget(&mut self, widget_id: (u64, u64)) {
        self.to_display
            .get_mut(&widget_id.0)
            .map(|v| v.remove(&widget_id.1));
    }
    ///Call this in the render function of your state to render every widget
    pub fn render<Store: Assets>(&self, assets: &Store, window: &mut Window) {
        let mut z = self.start_z;
        let widgets = Context::get_widgets(&self.to_display);
        widgets.iter().for_each(|(_, widget)| {
            widget.render(assets, window, z);
            z += 1;
        });
    }
    ///Adds a widget configuration to a given layer.
    ///
    ///Returns an Error if the layer does not exist.
    ///
    /// Otherwise, returns both the id of the widget AND a channel to comunicate with it.
    pub fn add_widget<R, W, Res>(&mut self, widget: R, layer_id: u64) -> Result<Response<Res>, ()>
    where
        R: WidgetConfig<Res, W>,
        W: Widget + 'a,
        Res: Sized,
    {
        match self.to_display.get_mut(&layer_id) {
            Some(layer) => {
                let (widget, res) = widget.to_widget();
                Ok(Response {
                    channel: res,
                    id: (layer_id, layer.insert(Box::new(widget))),
                })
            }
            _ => Err(()),
        }
    }
}
