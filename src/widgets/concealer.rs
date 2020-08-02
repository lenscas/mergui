use super::{button::Button, ButtonConfig, Widget, WidgetConfig};
use crate::channels::concealer::ConcealerReturn;
use quicksilver::graphics::Graphics;
use quicksilver::geom::Vector;
use quicksilver::{Window, Result};

//use quicksilver::prelude::{Vector, Window};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};
///A button that will hide/unhide other widgets when the user clicks on it
pub struct ConcealerConfig<T: PartialEq, R: Sized, W: Widget, E: WidgetConfig<R, W>> {
    ///A button that when clicked will hide/unhide the other widgets
    pub button: ButtonConfig,
    ///the widgets that can be hidden or shown
    pub hidden_widgets: Vec<(T, E)>,
    pub to_widget: PhantomData<W>,
    pub to_result: PhantomData<R>,
}

pub struct Concealer<W: Widget> {
    pub hidden_widgets: Vec<W>,
    pub button: Button,
    pub is_concealing: Arc<Mutex<bool>>,
}

impl<T: PartialEq, R: Sized, W: Widget, C: WidgetConfig<R, W>>
    WidgetConfig<ConcealerReturn<T, R>, Concealer<W>> for ConcealerConfig<T, R, W, C>
{
    fn to_widget(self) -> (Concealer<W>, ConcealerReturn<T, R>) {
        let mut widgets = Vec::new();
        let mut returns = Vec::new();
        self.hidden_widgets
            .into_iter()
            .map(|(name, widget_config)| (name, widget_config.to_widget()))
            .for_each(|(name, widget_and_res)| {
                returns.push((name, widget_and_res.1));
                widgets.push(widget_and_res.0);
            });
        let widgets = widgets;
        let returns = returns;
        let (main_button_widget, main_button_channel) = self.button.to_widget();
        let is_concealing = Arc::new(Mutex::new(true));
        (
            Concealer {
                is_concealing: Arc::clone(&is_concealing),
                hidden_widgets: widgets,
                button: main_button_widget,
            },
            ConcealerReturn {
                is_concealing,
                items: returns,
                main_button: main_button_channel,
            },
        )
    }
}
impl<W: Widget> Widget for Concealer<W> {
    fn contains(&self, point: &Vector) -> bool {
        if self.button.contains(point) {
            true
        } else if !self.is_concealing() {
            self.hidden_widgets.iter().any(|v| v.contains(point))
        } else {
            false
        }
    }
    fn is_focusable(&self, location: &Vector) -> bool {
        if self.is_concealing() {
            false
        } else {
            self.get_widget_at(*location)
                .map(|w| w.is_focusable(location))
                .unwrap_or(false)
        }
    }
    fn set_hover(&mut self, location: &Vector, hover: bool) {
        if self.button.contains(location) {
            self.button.set_hover(location, hover);
        } else if !self.is_concealing() {
            if let Some(v) = self.get_mut_widget_at(*location) {
                v.set_hover(location, hover)
            }
        }
    }
    fn render(&mut self, gfx: &mut Graphics, w: &Window) -> Result<()> {
        self.button.render(gfx, w)?;
        if !self.is_concealing() {
            self.hidden_widgets
                .iter_mut()
                .enumerate()
                .map(|(_, widget)| widget.render(gfx, w))
                .collect::<Result<_>>()?;
        }
        Ok(())
    }
    fn on_click(&mut self, clicked_on: &Vector) {
        if self.button.contains(clicked_on) {
            self.set_concealing(
                !self
                    .is_concealing
                    .lock()
                    .map(|v| *v)
                    .unwrap_or_else(|v| *v.into_inner()),
            );
            self.button.on_click(clicked_on);
        } else if !self.is_concealing() {
            if let Some(widget) = self.get_mut_widget_at(*clicked_on) {
                widget.on_click(clicked_on)
            }
        }
    }
    fn get_cursor_on_hover(&self, pos: &Vector) -> quicksilver::CursorIcon {
        if self.button.contains(pos) {
            self.button.get_cursor_on_hover(pos)
        } else {
            self.hidden_widgets
                .iter()
                .find(|v| v.contains(pos))
                .map(|v| v.get_cursor_on_hover(pos))
                .unwrap_or(quicksilver::CursorIcon::Default)
        }
    }
}

impl<W: Widget> Concealer<W> {
    fn get_widget_at(&self, location: Vector) -> Option<&W> {
        self.hidden_widgets.iter().find(|w| w.contains(&location))
    }
    fn get_mut_widget_at(&mut self, location: Vector) -> Option<&mut W> {
        self.hidden_widgets
            .iter_mut()
            .find(|w| w.contains(&location))
    }
    fn is_concealing(&self) -> bool {
        match self.is_concealing.lock() {
            Ok(x) => *x,
            Err(x) => *(x.into_inner()),
        }
    }
    pub fn set_concealing(&mut self, state: bool) {
        match self.is_concealing.lock() {
            Ok(mut x) => *x = state,
            Err(x) => {
                let mut x = x.into_inner();
                *x = state
            }
        }
    }
}
