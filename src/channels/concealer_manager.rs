use super::{BasicClickable, ConcealerReturn};
use std::{cell::RefCell, rc::Rc};

pub struct ConcealerManagerReturn<T: PartialEq, R: Sized> {
    channels: Vec<ConcealerReturn<T, R>>,
    shown: Rc<RefCell<Option<usize>>>,
}

impl<T: PartialEq, R: Sized> ConcealerManagerReturn<T, R> {
    pub fn new(channels: Vec<ConcealerReturn<T, R>>, shown: Rc<RefCell<Option<usize>>>) -> Self {
        Self { channels, shown }
    }
    ///set which concealer is active (if any)
    pub fn set_active_concealer(&mut self, new_active: Option<usize>) {
        self.shown.swap(&RefCell::new(new_active));
    }
    ///get which concealer is currently active
    pub fn get_current_active(&self) -> Option<usize> {
        *self.shown.borrow()
    }
    ///get an iterator over every channel
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Vec<(T, R)>> + 'a> {
        Box::new(self.channels.iter().map(|v| &v.items))
    }
    ///get a muteable iterator over every channel
    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Vec<(T, R)>> + 'a> {
        Box::new(self.channels.iter_mut().map(|v| &mut v.items))
    }
    ///get an iterator over every button
    pub fn buttons_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a BasicClickable> + 'a> {
        Box::new(self.channels.iter().map(|v| &v.main_button))
    }
    ///get a mutable iterator over every button
    pub fn buttons_iter_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut BasicClickable> + 'a> {
        Box::new(self.channels.iter_mut().map(|v| &mut v.main_button))
    }
}
