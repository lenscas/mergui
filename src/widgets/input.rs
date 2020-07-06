use crate::{
    channels::InputChannel,
    widgets::{widget_traits::WidgetConfig, Widget},
    FontStyle,
};
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::LayoutGlyph,
    graphics::{Color, Graphics},
    Result, Timer, Window,
};

#[derive(Clone)]
pub struct PlaceholderConfig {
    pub font: FontStyle,
    pub text: String,
}
pub struct CursorConfig {
    pub color: Color,
    pub thickness: f32,
    pub time_on: Timer,
    pub time_off: Timer,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl CursorConfig {
    pub fn new() -> Self {
        Self {
            color: Color::BLACK,
            thickness: 2.0,
            time_on: Timer::time_per_second(1.0),
            time_off: Timer::time_per_second(2.0),
        }
    }
}

pub struct InputConfig {
    pub font: FontStyle,
    pub placeholder: Option<PlaceholderConfig>,
    pub location: Rectangle,
    pub start_value: Option<String>,
    pub cursor_config: CursorConfig,
}

pub struct Input {
    config: InputConfig,
    value: InputChannel,
    cursor_at_from_left: usize,
    cursor_is_visible: bool,
    has_focus: bool,
}

impl WidgetConfig<InputChannel, Input> for InputConfig {
    fn to_widget(self) -> (Input, InputChannel) {
        let value = self.start_value.clone().unwrap_or_else(|| "".into()).into();
        (
            Input {
                config: self,
                value: InputChannel::clone(&value),
                cursor_at_from_left: 0,
                cursor_is_visible: true,
                has_focus: false,
            },
            value,
        )
    }
}

impl Input {
    fn get_glyphs(
        cursor_at: usize,
        gfx: &mut Graphics,
        text: &str,
        font: &FontStyle,
        _: f32,
    ) -> Result<(f32, f32, Vec<LayoutGlyph>)> {
        let mut glyphs = Vec::new();
        let mut length_before_cursor = 0.0;
        let cursor_at_from_left = cursor_at;
        font.font.layout_glyphs(gfx, &text, None, |_, glyph| {
            if glyphs.len() + 1 == cursor_at_from_left {
                length_before_cursor = glyph.position.x + glyph.glyph.bounds.width as f32;
            }
            glyphs.push(glyph);
        })?;
        let total_length = glyphs
            .last()
            .map(|glyph| glyph.position.x + glyph.glyph.bounds.width as f32)
            .unwrap_or(0.0);
        Ok((length_before_cursor, total_length, glyphs))
    }

    fn _calc_offset(max_size: f32, current_total_size: f32, size_before_cursor: f32) -> f32 {
        if current_total_size <= max_size {
            return 0.0;
        }
        if size_before_cursor <= max_size {
            return 0.0;
        }
        size_before_cursor - max_size
    }
    fn draw_text(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        let val = self.value.get();
        let (val, font) = if val == "" {
            match &mut self.config.placeholder {
                Some(v) => (v.text.as_str(), &mut v.font),
                None => ("", &mut self.config.font),
            }
        } else {
            (val.as_str(), &mut self.config.font)
        };

        let (size_before_cursor, _, glyphs) = Self::get_glyphs(
            self.cursor_at_from_left,
            gfx,
            val,
            font,
            self.config.location.x(),
        )?;
        glyphs.iter().for_each(|layout_glyph| {
            let glyph_bounds = layout_glyph.glyph.bounds;
            let pos = Vector::new(
                layout_glyph.position.x + self.config.location.pos.x,
                layout_glyph.position.y as f32
                    + self.config.font.font.size
                    + self.config.location.pos.y,
            );

            let glyph_size = Vector::new(glyph_bounds.width as f32, glyph_bounds.height as f32);
            let region = Rectangle::new(
                Vector::new(glyph_bounds.x as f32, glyph_bounds.y as f32),
                glyph_size,
            );

            gfx.draw_subimage_tinted(
                &layout_glyph.image,
                region,
                Rectangle::new(pos, glyph_size),
                self.config.font.color,
            );
        });
        if !self.has_focus {
            return Ok(());
        }
        if self.cursor_is_visible {
            if self.config.cursor_config.time_on.exhaust().is_some() {
                self.cursor_is_visible = false;
                self.config.cursor_config.time_off.reset();
            }
        } else if self.config.cursor_config.time_off.exhaust().is_some() {
            self.cursor_is_visible = true;
            self.config.cursor_config.time_on.reset();
        }
        if self.cursor_is_visible {
            gfx.fill_rect(
                &Rectangle::new(
                    Vector::new(
                        self.config.location.pos.x + size_before_cursor,
                        self.config.location.pos.y,
                    ),
                    Vector::new(
                        self.config.cursor_config.thickness,
                        self.config.location.size.y,
                    ),
                ),
                self.config.cursor_config.color,
            );
        }
        Ok(())
    }
}

impl Widget for Input {
    fn contains(&self, pos: Vector) -> bool {
        self.config.location.contains(Vector::new(pos.x, pos.y))
    }
    fn is_focusable(&self, _: Vector) -> bool {
        true
    }
    fn render(&mut self, gfx: &mut Graphics, window: &Window) -> Result<()> {
        gfx.stroke_rect(&self.config.location, Color::BLACK);
        self.draw_text(gfx, window)
    }
    fn set_focus(&mut self, _: Vector, focus: bool) {
        if focus {
            self.config.cursor_config.time_off.reset();
            self.config.cursor_config.time_on.reset();
            self.cursor_is_visible = true;
        } else {
            self.cursor_is_visible = false;
        }
        self.has_focus = focus
    }
    fn get_cursor_on_hover(&self, _: Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Text
    }

    fn on_key_press(&mut self, key: quicksilver::input::Key, state: bool) {
        use quicksilver::input::Key::*;
        if Back == key && state && self.cursor_at_from_left > 0 {
            let current_char_count = self.value.char_count();
            self.value.remove_char_at(self.cursor_at_from_left - 1);
            self.cursor_at_from_left -= current_char_count - self.value.char_count();
        }
        if key == Left && state && self.cursor_at_from_left > 0 {
            self.cursor_at_from_left -= 1;
        }
        if key == Right && state {
            let size = self.value.char_count();
            if self.cursor_at_from_left < size {
                self.cursor_at_from_left += 1;
            }
        }
    }

    fn on_typed(&mut self, typed_char: char) {
        if typed_char.is_control() {
            return;
        }
        let old_count = self.value.char_count();
        if self.cursor_at_from_left == old_count {
            self.value.push(typed_char)
        } else {
            self.value
                .insert_char_at_place(self.cursor_at_from_left, typed_char)
        };
        self.cursor_at_from_left += self.value.char_count() - old_count;
    }
}
