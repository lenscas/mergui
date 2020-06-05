use super::{ConcealerConfig, Widget, WidgetConfig};
use crate::{channels::ConcealerManagerReturn, widgets::concealer::Concealer};
use quicksilver::graphics::Graphics;
use quicksilver::{geom::Vector, Result, Window};
use std::{cell::RefCell, rc::Rc};

pub struct ConcealerManagerConfig {
    pub concealers: Vec<ConcealerConfig>,
}
pub struct ConcealerManager {
    concealers: Vec<Concealer>,
    pub active: Rc<RefCell<Option<usize>>>,
}
impl WidgetConfig<ConcealerManagerReturn, ConcealerManager> for ConcealerManagerConfig {
    fn to_widget(self) -> (ConcealerManager, ConcealerManagerReturn) {
        let mut channels = Vec::new();
        let mut widgets = Vec::new();
        self.concealers
            .into_iter()
            .map(WidgetConfig::to_widget)
            .for_each(|(widget, channel)| {
                channels.push(channel);
                widgets.push(widget);
            });
        let channels = channels;
        let widgets = widgets;
        let shown = Rc::new(RefCell::new(None));
        let manager = ConcealerManager {
            active: shown.clone(),
            concealers: widgets,
        };
        let channels = ConcealerManagerReturn::new(channels, shown);
        (manager, channels)
    }
}
impl ConcealerManager {
    fn get_hovered_mut(&mut self, pos: Vector) -> Option<&mut Concealer> {
        self.concealers.iter_mut().find(|v| v.contains(pos))
    }
    fn get_hovered(&self, pos: Vector) -> Option<&Concealer> {
        self.concealers.iter().find(|v| v.contains(pos))
    }
}
impl Widget for ConcealerManager {
    fn contains(&self, pos: Vector) -> bool {
        self.get_hovered(pos).map(|_| true).unwrap_or(false)
    }
    fn is_focusable(&self, _: Vector) -> bool {
        false
    }
    fn render(&mut self, gfx: &mut Graphics, w: &Window) -> Result<()> {
        self.concealers
            .iter_mut()
            .enumerate()
            .map(|(_, widget)| widget.render(gfx, w))
            .collect::<Result<_>>()
    }
    fn get_cursor_on_hover(&self, pos: Vector) -> quicksilver::CursorIcon {
        self.get_hovered(pos)
            .map(|v| v.get_cursor_on_hover(pos))
            .unwrap_or(quicksilver::CursorIcon::Default)
    }
    fn set_focus(&mut self, pos: Vector, state: bool) {
        if let Some(x) = self.get_hovered_mut(pos) {
            x.set_focus(pos, state)
        }
    }
    fn set_hover(&mut self, pos: Vector, state: bool) {
        if let Some(x) = self.get_hovered_mut(pos) {
            x.set_hover(pos, state)
        }
    }
    fn on_click(&mut self, pos: Vector) {
        let on_button = self
            .concealers
            .iter_mut()
            .enumerate()
            .find(|(_, widget)| widget.button.contains(pos));
        let mut current_active = self.active.borrow_mut();
        match (on_button, current_active.as_ref()) {
            (Some(button), Some(cur)) if button.0 != *cur => {
                let (key, widget) = button;
                widget.on_click(pos);
                widget.set_is_concealing(true);
                if let Some(cur_widget) = self.concealers.get_mut(*cur) {
                    cur_widget.set_is_concealing(false)
                }
                current_active.replace(key);
            }
            (Some(button), Some(cur)) if button.0 == *cur => {
                let (_, widget) = button;
                widget.on_click(pos);
                widget.set_is_concealing(false);
                *current_active = None;
            }
            (Some(button), None) => {
                let (key, widget) = button;
                widget.on_click(pos);
                widget.set_is_concealing(true);
                *current_active = Some(key);
            }
            (None, Some(cur)) => {
                if let Some(widget) = self.concealers.get_mut(*cur) {
                    widget.on_click(pos);
                }
            }
            (None, None) => {}
            (Some(_), Some(_)) => unreachable!("How....."),
        }
    }
}
