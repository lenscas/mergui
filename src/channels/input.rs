use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct InputChannel(Rc<RefCell<String>>);
impl InputChannel {
    pub fn new(v: String) -> Self {
        InputChannel(Rc::new(RefCell::new(v)))
    }
    pub fn get(&self) -> String {
        self.0.borrow().clone()
    }
    pub fn set(&mut self, new_val: String) {
        let mut v = self.0.borrow_mut();
        *v = new_val;
    }
}
impl From<String> for InputChannel {
    fn from(v: String) -> Self {
        Self::new(v)
    }
}
