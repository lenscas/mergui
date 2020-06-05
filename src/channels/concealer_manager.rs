use super::{BasicClickable, ConcealerReturn};
use crate::SingularLayerId;
use std::{cell::RefCell, rc::Rc};

pub struct ConcealerManagerReturn {
    channels: Vec<ConcealerReturn>,
    shown: Rc<RefCell<Option<usize>>>,
}

impl ConcealerManagerReturn {
    pub fn new(channels: Vec<ConcealerReturn>, shown: Rc<RefCell<Option<usize>>>) -> Self {
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

    ///Gets a specific layer. Usefull for if you want to interact with the layers
    pub fn get_layer(&self, id: usize) -> Option<&SingularLayerId> {
        self.channels.get(id).map(|v| &v.layer)
    }

    ///Gets a specific layer as a mutable reference. Usefull for if you want to interact with the layers
    pub fn get_layer_mut(&mut self, id: usize) -> Option<&mut SingularLayerId> {
        self.channels.get_mut(id).map(|v| &mut v.layer)
    }

    ///get an iterator over every layer
    pub fn iter(&self) -> Box<dyn Iterator<Item = &SingularLayerId> + '_> {
        Box::new(self.channels.iter().map(|v| &v.layer))
    }
    ///get a muteable iterator over every layer
    pub fn iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut SingularLayerId> + '_> {
        Box::new(self.channels.iter_mut().map(|v| &mut v.layer))
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
