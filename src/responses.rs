use std::sync::mpsc::{Receiver, Sender};
/// This is the struct that gets returned when a widget is added to the context.
pub struct Response<R> {
    ///This is used to comunicate with the widget
    pub channel: R,
    pub(crate) _id: WidgetId,
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

///Used to keep set to which layer a widget belongs to.
///Dropping this causes every widget in this layer to be removed.
pub struct LayerId {
    pub(crate) id: LayerNummerId,
    channel: LayerChannelSender,
}
impl LayerId {
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
impl Drop for LayerId {
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
