use super::{button::Button, ButtonConfig, Widget, WidgetConfig};
use crate::{channels::concealer::ConcealerReturn, Assets};
use quicksilver::prelude::{Vector, Window};
use std::marker::PhantomData;

///A button that will hide/unhide other widgets when the user clicks on it
pub struct ConcealerConfig<T: PartialEq, R: Sized, W: Widget, E: WidgetConfig<R, W>> {
    pub button: ButtonConfig,
    pub hidden_widgets: Vec<(T, E)>,
    pub to_widget: PhantomData<W>,
    pub to_result: PhantomData<R>,
}

pub struct Concealer<W: Widget> {
    pub hidden_widgets: Vec<W>,
    pub button: Button,
    pub is_concealing: bool,
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
        (
            Concealer {
                is_concealing: true,
                hidden_widgets: widgets,
                button: main_button_widget,
            },
            ConcealerReturn {
                is_concealing: true,
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
        } else if !self.is_concealing {
            self.hidden_widgets.iter().any(|v| v.contains(point))
        } else {
            false
        }
    }
    fn is_focusable(&self, location: &Vector) -> bool {
        if self.is_concealing {
            false
        } else {
            self.get_widget_at(location)
                .map(|w| w.is_focusable(location))
                .unwrap_or(false)
        }
    }
    fn set_hover(&mut self, location: &Vector, hover: bool) {
        if self.button.contains(location) {
            self.button.set_hover(location, hover);
        } else if !self.is_concealing {
            self.get_mut_widget_at(location)
                .map(|v| v.set_hover(location, hover));
        }
    }
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32) {
        self.button.render(assets, window, z);
        if !self.is_concealing {
            self.hidden_widgets
                .iter()
                .enumerate()
                .for_each(|(key, widget)| widget.render(assets, window, z + (key as u32)))
        }
    }
    fn on_click(&mut self, clicked_on: &Vector) {
        if self.button.contains(clicked_on) {
            self.is_concealing = !self.is_concealing;
            self.button.on_click(clicked_on);
        } else if !self.is_concealing {
            self.get_mut_widget_at(clicked_on)
                .map(|widget| widget.on_click(clicked_on));
        }
    }
    fn get_cursor_on_hover(&self) -> quicksilver::input::MouseCursor {
        quicksilver::input::MouseCursor::Hand
    }
}

impl<W: Widget> Concealer<W> {
    fn get_widget_at(&self, location: &Vector) -> Option<&W> {
        self.hidden_widgets.iter().find(|w| w.contains(location))
    }
    fn get_mut_widget_at(&mut self, location: &Vector) -> Option<&mut W> {
        self.hidden_widgets
            .iter_mut()
            .find(|w| w.contains(location))
    }
}
