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
use quicksilver::graphics::Graphics;
use quicksilver::graphics::{FontRenderer, LayoutGlyph, VectorFont};
use quicksilver::Result;
pub(crate) use responses::{
    LayerChannelReceiver, LayerChannelSender, LayerInstructions, LayerNummerId,
    WidgetChannelReceiver, WidgetChannelSender, WidgetInstruction, WidgetNummerId,
};
pub use responses::{LayerId, Response, WidgetId};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct MFont {
    pub(crate) renderer: Rc<RefCell<FontRenderer>>,
    pub(crate) size: f32,
}

impl MFont {
    pub async fn load_ttf(gfx: &Graphics, path: &'static str, size: f32) -> Result<MFont> {
        Self::from_font(&VectorFont::load(path).await?, gfx, size)
    }

    pub fn from_font(font: &VectorFont, gfx: &Graphics, font_size: f32) -> Result<MFont> {
        Ok(MFont {
            renderer: Rc::new(RefCell::new(font.to_renderer(gfx, font_size)?)),
            size: font_size,
        })
    }

    pub fn layout_glyphs(
        &self,
        gfx: &mut Graphics,
        text: &str,
        max_width: Option<f32>,
        callback: impl FnMut(&mut Graphics, LayoutGlyph),
    ) -> Result<Vector> {
        self.renderer
            .borrow_mut()
            .layout_glyphs(gfx, text, max_width, callback)
    }

    pub fn draw(
        &self,
        gfx: &mut Graphics,
        text: &str,
        color: Color,
        offset: Vector,
    ) -> Result<Vector> {
        self.renderer.borrow_mut().draw(gfx, text, color, offset)
    }
}

///a wrapper arround all the values needed to draw some text
#[derive(Clone)]
pub struct FontStyle {
    pub font: MFont,
    pub location: Vector,
    pub color: Color,
}
impl FontStyle {
    fn draw(&self, gfx: &mut Graphics, text: &str) -> Result<Vector> {
        let mut renderer = self.font.renderer.borrow_mut();
        renderer.draw(gfx, text, self.color, self.location)
    }
}

use std::sync::{Arc, Mutex, MutexGuard};
pub(crate) fn force_mutex<T>(val: &Arc<Mutex<T>>) -> MutexGuard<T> {
    val.lock().unwrap_or_else(|v| v.into_inner())
}
