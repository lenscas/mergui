use super::{BasicClickable, Clickable};

pub trait Concealer<T: PartialEq, R: Sized> {
    fn set_concealed(&mut self, new_consealed_state: bool);
    fn is_concealing(&self) -> bool;
    fn get_item(&self, key: T) -> Option<&R>;
    fn get_item_mut(&mut self, key: T) -> Option<&mut R>;
    fn iter_mut(&mut self) -> std::slice::IterMut<(T, R)>;
    fn iter(&self) -> std::slice::Iter<(T, R)>;
}
pub struct ConcealerReturn<T: PartialEq, R: Sized> {
    pub(crate) is_concealing: bool,
    pub(crate) items: Vec<(T, R)>,
    pub(crate) main_button: BasicClickable,
}
impl<T: PartialEq, R: Sized> Concealer<T, R> for ConcealerReturn<T, R> {
    fn set_concealed(&mut self, new_consealed_state: bool) {
        self.is_concealing = new_consealed_state;
    }
    fn is_concealing(&self) -> bool {
        self.is_concealing
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
impl<T: PartialEq, R: Sized> Clickable for ConcealerReturn<T, R> {
    fn has_clicked(&mut self) -> bool {
        self.main_button.has_clicked()
    }
}
