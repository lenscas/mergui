use crate::widgets::dropdown::DropDownValueConfig;
use std::{cell::RefCell, rc::Rc};
///Used to comunicate with the dropdown widget. It allows you to see if its open and to get the current selected value (if any)
pub struct Dropdown<T: Clone> {
    pub(crate) is_open: Rc<RefCell<bool>>,
    pub(crate) selected: Rc<RefCell<Option<usize>>>,
    pub(crate) values: Rc<RefCell<Vec<DropDownValueConfig<T>>>>,
}

impl<T: Clone> Dropdown<T> {
    pub fn get_value(&self) -> Option<T> {
        let selected = self.selected.borrow();
        let selected = selected.as_ref()?;
        let v = &self.values.borrow();
        v.get(*selected).map(|value| value.value.clone())
    }
    pub fn is_open(&self) -> bool {
        *self.is_open.borrow()
    }
}
