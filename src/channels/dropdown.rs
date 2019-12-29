use crate::{force_mutex, widgets::dropdown::DropDownValueConfig};
use std::sync::{Arc, Mutex};
///Used to comunicate with the dropdown widget. It allows you to see if its open and to get the current selected value (if any)
pub struct Dropdown<T: Clone> {
    pub(crate) is_open: Arc<Mutex<bool>>,
    pub(crate) selected: Arc<Mutex<Option<usize>>>,
    pub(crate) values: Arc<Mutex<Vec<DropDownValueConfig<T>>>>,
}

impl<T: Clone> Dropdown<T> {
    pub fn get_value(&self) -> Option<T> {
        let selected = (*force_mutex(&self.selected))?;
        let v = force_mutex(&self.values);
        v.get(selected).map(|value| value.value.clone())
    }
    pub fn is_open(&self) -> bool {
        *force_mutex(&self.is_open)
    }
}
