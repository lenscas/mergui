///Contains various structs and traits to comunicate with the widgets
pub mod channels;
mod context;
///Contains structs and traits of the most simple of widgets.
pub mod core;
mod responses;
///Contains the most used widgets and traits.
///Often they are composed of multiple core or normal widgets
pub mod widgets;

pub use crate::context::Context;
use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use quicksilver::graphics::Font;
use quicksilver::graphics::Graphics;
use quicksilver::Result;
pub(crate) use responses::{
    LayerChannelReceiver, LayerChannelSender, LayerInstructions, LayerNummerId,
    WidgetChannelReceiver, WidgetChannelSender, WidgetInstruction, WidgetNummerId,
};
pub use responses::{LayerId, Response, WidgetId};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct MFont(Rc<RefCell<Font>>);

impl From<Font> for MFont {
    fn from(from: Font) -> MFont {
        MFont(Rc::new(RefCell::new(from)))
    }
}
impl MFont {
    pub async fn load_ttf(gfx: &Graphics, path: &'static str) -> Result<MFont> {
        Ok(Font::load_ttf(gfx, path).await?.into())
    }
}

///a wrapper arround all the values needed to draw some text
#[derive(Clone)]
pub struct FontStyle {
    pub font: MFont,
    pub size: f32,
    pub location: Vector,
    pub color: Color,
    pub max_width: Option<f32>,
}

use std::sync::{Arc, Mutex, MutexGuard};
pub(crate) fn force_mutex<T>(val: &Arc<Mutex<T>>) -> MutexGuard<T> {
    val.lock().unwrap_or_else(|v| v.into_inner())
}
