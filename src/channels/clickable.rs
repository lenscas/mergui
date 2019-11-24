use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub trait Clickable {
    fn has_clicked(&mut self) -> bool;
}
pub struct BasicClickable {
    reader: Receiver<bool>,
}
impl Clickable for BasicClickable {
    fn has_clicked(&mut self) -> bool {
        self.reader
            .try_iter()
            .fold(false, |cur, message| cur || message)
    }
}
impl BasicClickable {
    pub fn new() -> (Self, ClickSetter) {
        let (writer, reader): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        (Self { reader }, writer.into())
    }
}
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
