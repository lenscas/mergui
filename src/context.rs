use crate::{
    widgets::{Widget, WidgetConfig},
    LayerChannelReceiver, LayerChannelSender, LayerId, LayerInstructions, LayerNummerId, Response,
    WidgetChannelReceiver, WidgetChannelSender, WidgetId, WidgetNummerId,
};
use indexmap::IndexMap;
use quicksilver::graphics::Graphics;
use quicksilver::lifecycle::MouseButton;
use quicksilver::mint::Vector2;
use quicksilver::{graphics::Image, lifecycle::Window};
use std::sync::mpsc;

struct Layer<'a> {
    is_active: bool,
    widgets: IndexMap<WidgetNummerId, Box<dyn Widget + 'a>>,
    current_id: LayerNummerId,
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
    to_display: IndexMap<LayerNummerId, Layer<'a>>,
    widget_with_focus: Option<(u64, u64)>,
    last_layer_id: u64,
    mouse_cursor: Vector2<f32>,
    layer_channel: LayerChannelReceiver,
    layer_channel_creator: LayerChannelSender,
    widget_channel: WidgetChannelReceiver,
    widget_channel_creator: WidgetChannelSender,
    left_mouse_button_down: bool,
}

impl<'a> Context<'a> {
    ///Due to bug [434](https://github.com/ryanisaacg/quicksilver/issues/434) in quicksilver, the draw order of draw calls without a "z" value is unspecified
    ///
    ///To get arround this, the context will draw every widget with its own Z value.
    ///
    ///It draws the first widget with start_z and increases it by one for every widget, resseting this back to start_z every frame
    pub fn new(cursor: Vector2<f32>, start_z: u32) -> Self {
        let (layer_send, layer_rec) = mpsc::channel();
        let (widget_send, widget_rec) = mpsc::channel();
        Self {
            to_display: IndexMap::new(),
            widget_with_focus: None,
            last_layer_id: 0,
            mouse_cursor: cursor,
            start_z,
            layer_channel: layer_rec,
            layer_channel_creator: layer_send,
            widget_channel: widget_rec,
            widget_channel_creator: widget_send,
            left_mouse_button_down: false,
        }
    }
    ///Adds a layer that can hold multiple widgets.
    ///Usefull to group widgets together that need to be removed at the same time
    pub fn add_layer(&mut self) -> LayerId {
        self.last_layer_id += 1;
        self.to_display
            .insert(self.last_layer_id, Default::default());

        LayerId::new(self.last_layer_id, self.layer_channel_creator.clone())
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
    fn handle_extern_events(&mut self) {
        self.handle_layer_events();
        self.handle_widget_events();
    }
    fn handle_widget_events(&mut self) {
        use crate::WidgetInstruction;
        let channel = &self.widget_channel;

        for event in channel.try_iter() {
            let id = event.0;
            let layer = id.0;
            let id = id.1;
            match event.1 {
                WidgetInstruction::Drop => {
                    self.to_display.get_mut(&layer).map(|v| v.remove(&id));
                }
            }
        }
    }
    fn handle_layer_events(&mut self) {
        let channel = &self.layer_channel;
        for event in channel.try_iter() {
            let id = event.0;
            match event.1 {
                LayerInstructions::Drop => {
                    self.to_display.remove(&id);
                }
                LayerInstructions::SetIsActive(state) => {
                    self.to_display.get_mut(&id).map(|v| v.is_active = state);
                }
            }
        }
    }
    ///Call this in the event function of the state to update every widget.
    pub fn event(&mut self, event: &quicksilver::lifecycle::Event, window: &Window) {
        self.handle_extern_events();
        use quicksilver::lifecycle::Event::*;
        match event {
            PointerMoved(val) => {
                let cursor_location = &self.mouse_cursor;
                let val = val.location();
                let mut widgets = Context::get_widgets_mut(&mut self.to_display);
                let mut widgets = widgets
                    .iter_mut()
                    .filter_map(|(_, widget)| {
                        let does_hover = widget.contains(&val);
                        if does_hover {
                            Some(widget)
                        } else {
                            widget.set_hover(cursor_location, false);
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let cursor = widgets
                    .pop()
                    .map(|widget| {
                        widget.set_hover(cursor_location, true);
                        widget.get_cursor_on_hover(cursor_location)
                    })
                    .unwrap_or(quicksilver::lifecycle::CursorIcon::Default);
                widgets
                    .iter_mut()
                    .for_each(|v| v.set_hover(cursor_location, false));

                window.set_cursor_icon(Some(cursor));
                self.mouse_cursor = val;
            }
            PointerInput(input) => {
                if input.button() != MouseButton::Left {
                    return;
                }
                match (input.is_down(), self.left_mouse_button_down) {
                    //it was already down, do not register as click
                    (true, true) => return,
                    //the button was released, we only have to mark it as such
                    (false, true) => {
                        self.left_mouse_button_down = false;
                        return;
                    }
                    //first time the button got pressed, mark it as such and continue prosessing the event
                    (true, false) => {
                        self.left_mouse_button_down = true;
                    }
                    //the button is released while never being pressed.
                    //this shouldn't happen
                    (false, false) => unreachable!(),
                }
                let cursor = &self.mouse_cursor;
                let mut widgets = Context::get_widgets_mut(&mut self.to_display);
                let mut maybe_focused_widgets: Vec<_> = widgets
                    .iter_mut()
                    .filter_map(|(id, widget)| {
                        let contains = widget.contains(cursor);
                        let is_focusable = widget.is_focusable(cursor);
                        if contains {
                            Some((id, widget, is_focusable))
                        } else {
                            widget.set_focus(cursor, false);
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
                                widget.set_focus(cursor, true)
                            };
                            widget.on_click(cursor);
                            (id.0, id.1)
                        });
                maybe_focused_widgets
                    .iter_mut()
                    .for_each(|(_, widget, is_focusable)| {
                        if *is_focusable {
                            widget.set_focus(cursor, false)
                        }
                    });
            }
            KeyboardInput(event) => {
                let key = event.key();
                let is_down = event.is_down();
                self.get_focused_widget()
                    .map(|focused| focused.on_key_press(key, is_down));
            }
            ReceivedCharacter(typed) => {
                self.get_focused_widget()
                    .map(|v| v.on_typed(typed.character()));
            }
            _ => {}
        }
    }
    ///Call this in the render function of your state to render every widget
    pub fn render<Store: Assets>(&mut self, assets: &Store, gfx: &mut Graphics) {
        self.handle_extern_events();
        let mut z = self.start_z;
        let widgets = Context::get_widgets(&self.to_display);
        widgets.iter().for_each(|(_, widget)| {
            widget.render(assets, gfx);
            z += 1;
        });
    }
    ///Adds a widget configuration to a given layer.
    ///
    ///Returns an Error if the layer does not exist.
    ///
    /// Otherwise, returns both the id of the widget AND a channel to comunicate with it.
    pub fn add_widget<R, W, Res>(
        &mut self,
        widget: R,
        layer_id: &LayerId,
    ) -> Result<Response<Res>, ()>
    where
        R: WidgetConfig<Res, W>,
        W: Widget + 'a,
        Res: Sized,
    {
        match self.to_display.get_mut(&layer_id.id) {
            Some(layer) => {
                let (widget, res) = widget.to_widget();
                Ok(Response {
                    channel: res,
                    id: WidgetId::new(
                        layer_id.id,
                        layer.insert(Box::new(widget)),
                        self.widget_channel_creator.clone(),
                    ),
                })
            }
            _ => Err(()),
        }
    }
}
