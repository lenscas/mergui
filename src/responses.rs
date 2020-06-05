use crate::widgets::{Widget, WidgetConfig};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};
/// This is the struct that gets returned when a widget is added to the context.
pub struct Response<R> {
    ///This is used to comunicate with the widget
    pub channel: R,
    pub(crate) _id: WidgetId,
    pub(crate) _layer_id: LayerId,
}

pub(crate) type LayerNummerId = u64;
pub(crate) type WidgetNummerId = u64;
pub(crate) type LayerChannelSender = Sender<(LayerNummerId, LayerInstructions)>;
pub(crate) type LayerChannelReceiver = Receiver<(LayerNummerId, LayerInstructions)>;

pub(crate) type WidgetChannelSender = Sender<((LayerNummerId, WidgetNummerId), WidgetInstruction)>;
pub(crate) type WidgetChannelReceiver =
    Receiver<((LayerNummerId, WidgetNummerId), WidgetInstruction)>;
pub(crate) enum LayerInstructions {
    Drop,
    AddWidget(Box<dyn Widget + 'static>, WidgetNummerId),
}

pub(crate) enum WidgetInstruction {
    Drop,
}

///The same as LayerId, but you can't clone this one
///Used for widgets that want to take control of a layer
///For example Widgets::Concealer
pub struct SingularLayerId(pub(crate) LayerId);

impl SingularLayerId {
    pub fn into_layer_id(self) -> LayerId {
        self.0
    }
    pub fn add_widget<ReturnChannel, W: Widget + 'static>(
        &mut self,
        widget_config: impl WidgetConfig<ReturnChannel, W>,
    ) -> Response<ReturnChannel> {
        self.0.add_widget(widget_config)
    }
    pub fn get_active(&self) -> bool {
        self.0.get_active()
    }

    pub(crate) fn set_is_active(&mut self, is_active: bool) {
        self.0.set_is_active(is_active)
    }
}

///Used to create widgets at a layer.
///Once this and every widget on this layer are dropped, so is the internal layer.
#[derive(Clone)]
pub struct LayerId {
    layer: Rc<InternalLayerId>,
    pub(crate) is_active: Rc<RefCell<bool>>,
    widget_id: Rc<RefCell<WidgetNummerId>>,
    widget_channel: WidgetChannelSender,
}

impl LayerId {
    pub(crate) fn new(
        id: LayerNummerId,
        channel: LayerChannelSender,
        is_active: Rc<RefCell<bool>>,
        widget_id: Rc<RefCell<WidgetNummerId>>,
        widget_channel: WidgetChannelSender,
    ) -> Self {
        let layer = Rc::new(InternalLayerId::new(id, channel));
        Self {
            layer,
            is_active,
            widget_id,
            widget_channel,
        }
    }

    ///Adds a widget configuration to the layer that this id represents.
    ///Returns a channel to comunicate with the widget.
    pub fn add_widget<ReturnChannel, W: Widget + 'static>(
        &mut self,
        widget_config: impl WidgetConfig<ReturnChannel, W>,
    ) -> Response<ReturnChannel> {
        let (widget, res) = widget_config.to_widget();
        let mut widget_id = self.widget_id.borrow_mut();
        *widget_id += 1;
        let x = Response {
            _layer_id: self.clone(),
            channel: res,
            _id: WidgetId::new(self.id(), *widget_id, self.widget_channel.clone()),
        };
        self.layer.as_ref().add_widget(widget, *widget_id);
        x
    }

    pub(crate) fn id(&self) -> LayerNummerId {
        self.layer.as_ref().id
    }

    ///Set a layer to active or inactive.
    ///Layers that are inactive won't be rendered or receive updates.
    pub fn set_is_active(&self, is_active: bool) {
        self.is_active.replace(is_active);
    }

    ///Get if the layer is active or not.
    pub fn get_active(&self) -> bool {
        *self.is_active.borrow()
    }
}
pub(crate) struct InternalLayerId {
    pub(crate) id: LayerNummerId,
    channel: LayerChannelSender,
}

impl InternalLayerId {
    pub(crate) fn new(id: LayerNummerId, channel: LayerChannelSender) -> Self {
        Self { id, channel }
    }
    pub(crate) fn add_widget(&self, widget: impl Widget + 'static, id: WidgetNummerId) {
        let _ = self
            .channel
            .send((self.id, LayerInstructions::AddWidget(Box::new(widget), id)));
    }
}
impl Drop for InternalLayerId {
    fn drop(&mut self) {
        let _ = self.channel.send((self.id, LayerInstructions::Drop));
    }
}
pub(crate) struct WidgetId {
    pub(crate) id: WidgetNummerId,
    pub(crate) layer: LayerNummerId,
    channel: WidgetChannelSender,
}
impl Drop for WidgetId {
    fn drop(&mut self) {
        let _ = self
            .channel
            .send(((self.layer, self.id), WidgetInstruction::Drop));
    }
}
impl WidgetId {
    pub(crate) fn new(
        layer: LayerNummerId,
        id: WidgetNummerId,
        channel: WidgetChannelSender,
    ) -> Self {
        Self { layer, id, channel }
    }
}
