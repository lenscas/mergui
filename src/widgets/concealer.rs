use super::{button::Button, ButtonConfig, Widget, WidgetConfig};
use crate::{channels::concealer::ConcealerReturn, SingularLayerId};
use quicksilver::graphics::Graphics;
use quicksilver::{geom::Vector, Result, Window};

//use quicksilver::prelude::{Vector, Window};
use std::{cell::RefCell, rc::Rc};
///A button that will hide/unhide other widgets when the user clicks on it
pub struct ConcealerConfig {
    ///A button that when clicked will hide/unhide the other widgets
    pub button: ButtonConfig,
    pub layer: SingularLayerId,
}

pub struct Concealer {
    pub button: Button,
    pub is_active: Rc<RefCell<bool>>,
}

impl WidgetConfig<ConcealerReturn, Concealer> for ConcealerConfig {
    fn to_widget(mut self) -> (Concealer, ConcealerReturn) {
        self.layer.set_is_active(false);
        let (main_button_widget, main_button_channel) = self.button.to_widget();
        let is_concealing = self.layer.0.is_active.clone();

        (
            Concealer {
                is_active: Rc::clone(&is_concealing),
                button: main_button_widget,
            },
            ConcealerReturn {
                layer: self.layer,
                main_button: main_button_channel,
            },
        )
    }
}
impl Widget for Concealer {
    fn contains(&self, point: Vector) -> bool {
        self.button.contains(point)
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn set_hover(&mut self, location: Vector, hover: bool) {
        self.button.set_hover(location, hover);
    }
    fn render(&mut self, gfx: &mut Graphics, w: &Window) -> Result<()> {
        self.button.render(gfx, w)?;
        Ok(())
    }
    fn on_click(&mut self, clicked_on: Vector) {
        let current_state = self.is_active();
        self.set_is_concealing(!current_state);
        self.button.on_click(clicked_on);
    }
    fn get_cursor_on_hover(&self, pos: Vector) -> quicksilver::CursorIcon {
        self.button.get_cursor_on_hover(pos)
    }
}

impl Concealer {
    fn is_active(&self) -> bool {
        *self.is_active.borrow()
    }
    pub fn set_is_concealing(&mut self, state: bool) {
        self.is_active.swap(&RefCell::new(state));
    }
}
