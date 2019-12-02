use super::{BasicClickable, Clickable};
use std::sync::{Arc, Mutex};

///A trait for every channel which can be used to hide/unhide multiple other widgets
pub trait Concealer<T: PartialEq, R: Sized> {
    ///set if the widgets should be hidden or shown
    fn set_concealed(&mut self, new_consealed_state: bool);
    ///get if the widgets are hidden or shown
    fn is_concealing(&self) -> bool;
    ///get a specific channel
    fn get_item(&self, key: T) -> Option<&R>;
    ///get a specific channel, mutable
    fn get_item_mut(&mut self, key: T) -> Option<&mut R>;
    ///get an iterator over every channel, mutable
    fn iter_mut(&mut self) -> std::slice::IterMut<(T, R)>;
    ///get an iterator over every channel
    fn iter(&self) -> std::slice::Iter<(T, R)>;
}
///A basic implementation of the Concealer channel. Used by the Concealer widget
pub struct ConcealerReturn<T: PartialEq, R: Sized> {
    pub(crate) is_concealing: Arc<Mutex<bool>>,
    pub(crate) items: Vec<(T, R)>,
    pub(crate) main_button: BasicClickable,
}
impl<T: PartialEq, R: Sized> Concealer<T, R> for ConcealerReturn<T, R> {
    fn set_concealed(&mut self, new_consealed_state: bool) {
        let locked = self.is_concealing.lock();
        match locked {
            Ok(mut res) => *res = new_consealed_state,
            Err(_) => {}
        }
    }
    fn is_concealing(&self) -> bool {
        let locked = self.is_concealing.lock();
        match locked {
            Ok(res) => res.clone(),
            Err(x) => x.into_inner().clone(),
        }
    }
    fn get_item(&self, key: T) -> Option<&R> {
        self.items
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, res)| res)
    }
    fn get_item_mut(&mut self, key: T) -> Option<&mut R> {
        self.items
            .iter_mut()
            .find(|(name, _)| *name == key)
            .map(|(_, res)| res)
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<(T, R)> {
        self.items.iter_mut()
    }
    fn iter(&self) -> std::slice::Iter<(T, R)> {
        self.items.iter()
    }
}

///This is hooked up to the button that hides/unhides the widgets.
impl<T: PartialEq, R: Sized> Clickable for ConcealerReturn<T, R> {
    fn has_clicked(&mut self) -> bool {
        self.main_button.has_clicked()
    }
}
