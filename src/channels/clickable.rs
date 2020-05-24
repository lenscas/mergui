use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

///A trait implemented by every channel that registers left clicks
pub trait Clickable {
    fn has_clicked(&mut self) -> bool;
}
///A simple struct that implements the Clickable trait.
///Used by buttons and other widgets that only return back that they got clicked
pub struct BasicClickable {
    reader: Receiver<bool>,
}
impl Clickable for BasicClickable {
    ///Returns true if the user clicked on the widget since the last time this function got called.
    fn has_clicked(&mut self) -> bool {
        self.reader.try_iter().any(|message| message)
    }
}
impl BasicClickable {
    pub fn new() -> (Self, ClickSetter) {
        let (writer, reader): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        (Self { reader }, writer.into())
    }
    ///Returns true if the user clicked on the widget since the last time this function got called.
    pub fn has_clicked(&mut self) -> bool {
        <Self as Clickable>::has_clicked(self)
    }
}

///A simple struct that can be used by widgets to update their channel.
pub struct ClickSetter(Sender<bool>);
impl ClickSetter {
    pub fn clicked(&mut self) {
        let _ = self.0.send(true); //It is not our problem if this widget never got cleared up.
    }
}
impl From<Sender<bool>> for ClickSetter {
    fn from(sender: Sender<bool>) -> Self {
        Self(sender)
    }
}
