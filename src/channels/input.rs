use std::{cell::RefCell, rc::Rc};

use unicode_normalization::UnicodeNormalization;

#[derive(Clone)]
pub struct InputChannel(Rc<RefCell<String>>);
impl InputChannel {
    pub fn new(v: String) -> Self {
        let mut channel = InputChannel(Rc::new(RefCell::new(v)));
        channel.normalize();
        channel
    }
    ///Get the current value
    ///
    ///```
    /// # use mergui::channels::InputChannel;
    /// let channel = InputChannel::new("abcd".into());
    /// assert_eq!(channel.get(), String::from("abcd"));
    ///```
    pub fn get(&self) -> String {
        self.0.borrow().clone()
    }

    ///Set a new value for the input.
    ///Note: the string will be normalized before it is set to prevent issues when drawing
    ///
    ///```
    /// # use mergui::channels::InputChannel;
    /// let mut channel = InputChannel::new("qwerty".into());
    /// channel.set("abcd".into());
    /// assert_eq!(channel.get(), String::from("abcd"));
    ///```
    pub fn set(&mut self, new_val: String) {
        self.0.replace(new_val);
        self.normalize();
    }

    ///Pushes a character to the end of this string. Then, normalize the value
    ///
    ///```
    ///# use mergui::channels::InputChannel;
    ///let mut channel = InputChannel::new("abc".into());
    ///channel.push('d');
    ///assert_eq!(channel.get(), String::from("abcd"));
    ///```
    pub fn push(&mut self, to_add: char) {
        let mut string = self.0.borrow_mut();
        string.push(to_add);
        drop(string);
        self.normalize();
    }

    /// Inserts a character at the given place. Then, normalize the value
    ///
    ///```
    ///# use mergui::channels::InputChannel;
    ///let mut channel = InputChannel::new("abd".into());
    ///channel.insert_char_at_place(2,'c');
    ///assert_eq!(channel.get(), String::from("abcd"));
    ///```
    pub fn insert_char_at_place(&mut self, char_index: usize, to_add: char) {
        let mut old_str = self.0.borrow_mut();
        let mut new_str = String::with_capacity(old_str.capacity());
        for (key, value) in old_str.chars().enumerate() {
            if key == char_index {
                new_str.push(to_add);
            }
            new_str.push(value)
        }
        *old_str = new_str;
        drop(old_str);
        self.normalize();
    }

    /// Removes a character at the given place. Then, normalize the value
    ///
    ///```
    ///# use mergui::channels::InputChannel;
    ///let mut channel = InputChannel::new("abdc".into());
    ///channel.remove_char_at(2);
    ///assert_eq!(channel.get(), String::from("abc"));
    ///```
    pub fn remove_char_at(&mut self, index: usize) {
        let new_value = self
            .0
            .borrow()
            .chars()
            .enumerate()
            .map(|(key, character)| (key, character))
            .filter(|(key, _)| key != &index)
            .map(|(_, character)| character)
            .collect();
        self.set(new_value);
    }

    ///Calculates the amount of characters inside a string.
    ///
    ///```
    ///# use mergui::channels::InputChannel;
    ///let channel = InputChannel::new("abc".into());
    ///assert_eq!(channel.char_count(), 3);
    ///```
    pub fn char_count(&self) -> usize {
        self.0.borrow().chars().count()
    }

    ///Get every character inside the string
    ///
    ///```
    ///# use mergui::channels::InputChannel;
    ///let channel = InputChannel::new("abc".into());
    ///assert_eq!(channel.chars(), vec!['a','b','c']);
    ///```
    pub fn chars(&self) -> Vec<char> {
        self.0.borrow().chars().collect()
    }

    fn normalize(&mut self) {
        self.0.replace_with(|v| v.nfc().collect());
    }
}
impl From<String> for InputChannel {
    fn from(v: String) -> Self {
        Self::new(v)
    }
}
