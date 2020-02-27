use super::{ConcealerConfig, Widget, WidgetConfig};
use crate::{channels::ConcealerManagerReturn, widgets::concealer::Concealer, Assets};
use quicksilver::graphics::Graphics;
use quicksilver::lifecycle::Window;
use quicksilver::mint::Vector2;
use std::sync::{Arc, Mutex};

pub struct ConcealerManagerConfig<T: PartialEq, R: Sized, W: Widget, E: WidgetConfig<R, W>> {
    pub concealers: Vec<ConcealerConfig<T, R, W, E>>,
}
pub struct ConcealerManager<W: Widget> {
    concealers: Vec<Concealer<W>>,
    pub active: Arc<Mutex<Option<usize>>>,
}
impl<T: PartialEq, R: Sized, W: Widget, C: WidgetConfig<R, W>>
    WidgetConfig<ConcealerManagerReturn<T, R>, ConcealerManager<W>>
    for ConcealerManagerConfig<T, R, W, C>
{
    fn to_widget(self) -> (ConcealerManager<W>, ConcealerManagerReturn<T, R>) {
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
        let shown = Arc::new(Mutex::new(None));
        let manager = ConcealerManager {
            active: shown.clone(),
            concealers: widgets,
        };
        let channels = ConcealerManagerReturn::new(channels, shown);
        (manager, channels)
    }
}
impl<W: Widget> ConcealerManager<W> {
    fn get_hovered_mut(&mut self, pos: &Vector2<f32>) -> Option<&mut Concealer<W>> {
        self.concealers.iter_mut().find(|v| v.contains(pos))
    }
    fn get_hovered(&self, pos: &Vector2<f32>) -> Option<&Concealer<W>> {
        self.concealers.iter().find(|v| v.contains(pos))
    }
}
impl<W: Widget> Widget for ConcealerManager<W> {
    fn contains(&self, pos: &Vector2<f32>) -> bool {
        self.get_hovered(pos).map(|_| true).unwrap_or(false)
    }
    fn is_focusable(&self, pos: &Vector2<f32>) -> bool {
        self.get_hovered(pos)
            .map(|v| v.is_focusable(pos))
            .unwrap_or(false)
    }
    fn render(&self, assets: &dyn Assets, gfx: &mut Graphics, z: u32) {
        self.concealers
            .iter()
            .enumerate()
            .for_each(|(key, widget)| widget.render(assets, gfx, z + (key as u32)))
    }
    fn get_cursor_on_hover(&self, pos: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        self.get_hovered(pos)
            .map(|v| v.get_cursor_on_hover(pos))
            .unwrap_or(quicksilver::lifecycle::CursorIcon::Default)
    }
    fn set_focus(&mut self, pos: &Vector2<f32>, state: bool) {
        match self.get_hovered_mut(pos) {
            Some(x) => x.set_focus(pos, state),
            None => {}
        }
    }
    fn set_hover(&mut self, pos: &Vector2<f32>, state: bool) {
        match self.get_hovered_mut(pos) {
            Some(x) => x.set_hover(pos, state),
            None => {}
        }
    }
    fn on_click(&mut self, pos: &Vector2<f32>) {
        let on_button = self
            .concealers
            .iter_mut()
            .enumerate()
            .find(|(_, widget)| widget.button.contains(pos));
        let mut current_active = self
            .active
            .lock()
            .map(|v| v)
            .unwrap_or_else(|v| v.into_inner());
        match (on_button, current_active.as_ref()) {
            (Some(button), Some(cur)) if button.0 != *cur => {
                let (key, widget) = button;
                widget.on_click(pos);
                widget.set_concealing(false);
                if let Some(cur_widget) = self.concealers.get_mut(*cur) {
                    cur_widget.set_concealing(true)
                }
                *current_active = Some(key);
            }
            (Some(button), Some(cur)) if button.0 == *cur => {
                let (_, widget) = button;
                widget.on_click(pos);
                widget.set_concealing(true);
                *current_active = None;
            }
            (Some(button), None) => {
                let (key, widget) = button;
                widget.on_click(pos);
                widget.set_concealing(false);
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
