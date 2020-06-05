use crate::{
    widgets::{Widget, WidgetConfig},
    LayerChannelReceiver, LayerChannelSender, LayerId, LayerInstructions, LayerNummerId, Response,
    SingularLayerId, WidgetChannelReceiver, WidgetChannelSender, WidgetId, WidgetNummerId,
};
use indexmap::IndexMap;
use quicksilver::{
    geom::Vector, graphics::Graphics, input::MouseButton, Result as QuickResult, Window,
};
use std::{cell::RefCell, rc::Rc, sync::mpsc};

struct Layer {
    is_active: Rc<RefCell<bool>>,
    widgets: IndexMap<WidgetNummerId, Box<dyn Widget + 'static>>,
    current_id: Rc<RefCell<LayerNummerId>>,
}
impl Default for Layer {
    fn default() -> Self {
        Self::new()
    }
}
impl Layer {
    pub fn new() -> Self {
        Self {
            is_active: Rc::new(RefCell::new(true)),
            widgets: Default::default(),
            current_id: Rc::new(RefCell::new(0)),
        }
    }
    pub fn get_mut(&mut self, index: u64) -> Option<&mut (dyn Widget + 'static)> {
        self.widgets.get_mut(&index).map(|v| v.as_mut())
    }
    pub fn remove(&mut self, index: u64) {
        self.widgets.remove(&index);
    }
    pub fn insert(&mut self, widget: Box<dyn Widget + 'static>) -> u64 {
        let mut id = self.current_id.borrow_mut();
        *id += 1;
        self.widgets.insert(*id, widget);
        *id
    }
    pub(crate) fn is_active(&self) -> bool {
        *self.is_active.borrow()
    }
}

///This manages the GUI. It contains every widget that needs to be drawn and makes sure they are updated properly
pub struct Context {
    to_display: IndexMap<LayerNummerId, Layer>,
    widget_with_focus: Option<(u64, u64)>,
    last_layer_id: u64,
    mouse_cursor: Vector,
    layer_channel: LayerChannelReceiver,
    layer_channel_creator: LayerChannelSender,
    widget_channel: WidgetChannelReceiver,
    widget_channel_creator: WidgetChannelSender,
    left_mouse_button_down: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        let (layer_send, layer_rec) = mpsc::channel();
        let (widget_send, widget_rec) = mpsc::channel();
        Self {
            to_display: IndexMap::new(),
            widget_with_focus: None,
            last_layer_id: 0,
            mouse_cursor: Vector::new(0., 0.),
            layer_channel: layer_rec,
            layer_channel_creator: layer_send,
            widget_channel: widget_rec,
            widget_channel_creator: widget_send,
            left_mouse_button_down: false,
        }
    }

    ///same as Context::add_layer but it gives you a layer back that can't be cloned.
    ///These are used by widgets that want to take control of an entire layer
    ///An example is Widgets::Concealer
    pub fn add_singular_layer(&mut self) -> SingularLayerId {
        let layer_id = self.add_layer();
        SingularLayerId { 0: layer_id }
    }

    ///Adds a layer that can hold multiple widgets.
    ///Usefull to group widgets together that need to be removed at the same time
    pub fn add_layer(&mut self) -> LayerId {
        self.last_layer_id += 1;
        let layer = Layer::new();
        let id = LayerId::new(
            self.last_layer_id,
            self.layer_channel_creator.clone(),
            layer.is_active.clone(),
            layer.current_id.clone(),
            self.widget_channel_creator.clone(),
        );
        self.to_display.insert(self.last_layer_id, layer);
        id
    }

    fn get_focused_widget(&mut self) -> Option<&mut (dyn Widget + 'static)> {
        self.widget_with_focus
            .and_then(move |v| self.to_display.get_mut(&v.0).and_then(|x| x.get_mut(v.1)))
    }

    fn get_widgets_mut<'b>(
        widgets: &'b mut IndexMap<u64, Layer>,
    ) -> Vec<((u64, u64), &'b mut (dyn Widget + 'static))> {
        widgets
            .iter_mut()
            .filter(|(_, layer)| layer.is_active())
            .flat_map(|(layer_id, layer)| {
                layer
                    .widgets
                    .iter_mut()
                    .map(move |(widget_id, widget)| ((layer_id, widget_id), widget))
            })
            .map(|(id, widget)| ((*id.0, *id.1), widget.as_mut()))
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
                    if let Some(v) = self.to_display.get_mut(&layer) {
                        v.remove(id)
                    }
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
                LayerInstructions::AddWidget(widget, widget_id) => {
                    if let Some(layer) = self.to_display.get_mut(&id) {
                        layer.widgets.insert(widget_id, widget);
                    }
                }
            }
        }
    }
    ///Call this in the event function of the state to update every widget.
    pub fn event(&mut self, event: &quicksilver::input::Event, window: &Window) {
        self.handle_extern_events();
        use quicksilver::input::Event::*;
        match event {
            PointerMoved(val) => {
                let cursor_location = self.mouse_cursor;
                let val = val.location();
                let mut widgets = Context::get_widgets_mut(&mut self.to_display);
                let mut widgets = widgets
                    .iter_mut()
                    .filter_map(|(_, widget)| {
                        let does_hover = widget.contains(val);
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
                    .unwrap_or(quicksilver::CursorIcon::Default);
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
                    //the button is released while never being pressed. I would've marked it as unreacable, but apparently it can happen?
                    //so.. instead lets do nothing....
                    (false, false) => return,
                }
                let cursor = self.mouse_cursor;
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
                let cursor = self.mouse_cursor;
                let current_focused_id = self.widget_with_focus;
                self.widget_with_focus =
                    maybe_focused_widgets
                        .pop()
                        .map(|(id, widget, is_focusable)| {
                            if is_focusable
                                && current_focused_id
                                    .map(|(layer, widget)| id.0 != layer || id.1 != widget)
                                    .unwrap_or(true)
                            {
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
                if let Some(focused) = self.get_focused_widget() {
                    focused.on_key_press(key, is_down)
                }
            }
            ReceivedCharacter(typed) => {
                if let Some(v) = self.get_focused_widget() {
                    v.on_typed(typed.character())
                }
            }
            _ => {}
        }
    }
    ///Call this in the render function of your state to render every widget
    pub fn render(&mut self, gfx: &mut Graphics, window: &Window) -> QuickResult<()> {
        self.handle_extern_events();
        let mut widgets = Context::get_widgets_mut(&mut self.to_display);
        widgets
            .iter_mut()
            .map(|(_, widget)| widget.render(gfx, window))
            .collect()
    }
    ///Adds a widget configuration to a given layer.
    ///
    ///Returns an Error if the layer does not exist.
    ///
    ///Otherwise, returns a channel to comunicate with the new widget.
    ///Note: You can also add a widget using LayerId::add_widget.
    pub fn add_widget<R, W, Res>(
        &mut self,
        widget: R,
        layer_id: &LayerId,
    ) -> Result<Response<Res>, ()>
    where
        R: WidgetConfig<Res, W>,
        W: Widget + 'static,
        Res: Sized,
    {
        match self.to_display.get_mut(&layer_id.id()) {
            Some(layer) => {
                let (widget, res) = widget.to_widget();
                Ok(Response {
                    _layer_id: layer_id.clone(),
                    channel: res,
                    _id: WidgetId::new(
                        layer_id.id(),
                        layer.insert(Box::new(widget)),
                        self.widget_channel_creator.clone(),
                    ),
                })
            }
            _ => Err(()),
        }
    }
}
