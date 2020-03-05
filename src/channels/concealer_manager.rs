use super::{BasicClickable, ConcealerReturn};
use std::sync::{Arc, Mutex};

pub struct ConcealerManagerReturn<T: PartialEq, R: Sized> {
    channels: Vec<ConcealerReturn<T, R>>,
    shown: Arc<Mutex<Option<usize>>>,
}

impl<T: PartialEq, R: Sized> ConcealerManagerReturn<T, R> {
    pub fn new(channels: Vec<ConcealerReturn<T, R>>, shown: Arc<Mutex<Option<usize>>>) -> Self {
        Self { channels, shown }
    }
    pub fn set_active_concealed(&mut self, new_active: Option<usize>) {
        let locked = self.shown.lock();
        if let Ok(mut x) = locked {
            *x = new_active
        }
    }
    pub fn get_current_active(&self) -> Option<usize> {
        let locked = self.shown.lock();
        match locked {
            Ok(res) => *res,
            Err(x) => *(x.into_inner()),
        }
    }
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Vec<(T, R)>> + 'a> {
        Box::new(self.channels.iter().map(|v| &v.items))
    }
    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Vec<(T, R)>> + 'a> {
        Box::new(self.channels.iter_mut().map(|v| &mut v.items))
    }
    pub fn buttons_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a BasicClickable> + 'a> {
        Box::new(self.channels.iter().map(|v| &v.main_button))
    }
    pub fn buttons_iter_mut<'a>(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = &'a mut BasicClickable> + 'a> {
        Box::new(self.channels.iter_mut().map(|v| &mut v.main_button))
    }
}
