use std::{
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
    SetIsActive(bool),
}

pub(crate) enum WidgetInstruction {
    Drop,
}

///Used to create widgets at a layer.
///Once this and every widget on this layer are dropped, so is the internal layer.
#[derive(Clone)]
pub struct LayerId {
    layer: Rc<InternalLayerId>,
}

impl LayerId {
    pub(crate) fn new(id: LayerNummerId, channel: LayerChannelSender) -> Self {
        let layer = Rc::new(InternalLayerId::new(id, channel));
        Self { layer }
    }

    pub(crate) fn id(&self) -> LayerNummerId {
        self.layer.as_ref().id
    }

    pub fn set_is_active(&self, is_active: bool) {
        self.layer.as_ref().set_is_active(is_active)
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

    ///Set a layer to active or inactive.
    ///Layers that are inactive won't be rendered or receive updates.
    pub fn set_is_active(&self, is_active: bool) {
        let _ = self
            .channel
            .send((self.id, LayerInstructions::SetIsActive(is_active)));
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
