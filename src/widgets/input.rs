use crate::widgets::{widget_traits::WidgetConfig, Widget};
use crate::{channels::InputChannel, FontStyle};
use quicksilver::geom::{Rectangle, Shape, Vector};
use quicksilver::{
    graphics::{Image, LayoutGlyph, PixelFormat, Surface},
    Window,
    Result,
    {
        graphics::{Color, Graphics},
    },
};

pub struct PlaceholderConfig {
    pub font: FontStyle,
    pub text: String,
}
pub struct InputConfig {
    pub font: FontStyle,
    pub placeholder: Option<PlaceholderConfig>,
    pub location: Rectangle,
    pub start_value: Option<String>,
}

pub struct Input {
    config: InputConfig,
    value: InputChannel,
    cursor_at_from_left: usize,
}

impl WidgetConfig<InputChannel, Input> for InputConfig {
    fn to_widget(self) -> (Input, InputChannel) {
        let value = self.start_value.clone().unwrap_or("".into()).into();
        (
            Input {
                config: self,
                value: InputChannel::clone(&value),
                cursor_at_from_left: 0,
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
        x_offset: f32,
    ) -> Result<(f32, f32, Vec<LayoutGlyph>)> {
        let mut glyphs = Vec::new();
        let mut length_before_cursor = 0.0;
        let cursor_at_from_left = cursor_at;
        font.font.layout_glyphs(gfx, &text, None, |_, glyph| {
            if glyphs.len() + 1 == cursor_at_from_left {
                length_before_cursor =
                    glyph.position.x + glyph.glyph.bounds.width as f32 - x_offset;
            }
            glyphs.push(glyph);
        })?;
        let total_length = glyphs
            .last()
            .map(|glyph| glyph.position.x + glyph.glyph.bounds.width as f32 - x_offset)
            .unwrap_or(0.0);
        Ok((length_before_cursor, total_length, glyphs))
    }
    /*
    fn select_glyphs_to_draw(
        &self,
        total_length: f32,
        mut length_before_cursor: f32,
        glyphs: Vec<LayoutGlyph>,
        x_offset: f32,
    ) -> Vec<(usize, LayoutGlyph)> {
        let max_size = self.config.location.width();
        if total_length as f32 > max_size {
            if (length_before_cursor as f32) < max_size {
                let mut current_size = length_before_cursor;
                glyphs
                    .into_iter()
                    .enumerate()
                    .take_while(|(key, val)| {
                        if key < &self.cursor_at_from_left {
                            true
                        } else {
                            current_size += val.glyph.bounds.width;
                            (current_size as f32) < max_size
                        }
                    })
                    .collect()
            } else {
                glyphs
                    .into_iter()
                    .enumerate()
                    .skip_while(|(_, v)| {
                        length_before_cursor -= v.glyph.bounds.width;
                        println!(
                            "length before {}, max length {}",
                            length_before_cursor, max_size
                        );
                        (length_before_cursor as f32) > max_size
                    })
                    .collect()
            }
        } else {
            glyphs.into_iter().enumerate().collect()
        }
    }
    */
    fn calc_offset(max_size: f32, current_total_size: f32, size_before_cursor: f32) -> f32 {
        if current_total_size <= max_size {
            return 0.0;
        }
        if size_before_cursor <= max_size {
            return 0.0;
        }
        size_before_cursor - max_size
    }
    fn draw_text(&mut self, gfx: &mut Graphics, window: &Window) -> Result<()> {
        let val = self.value.get();
        let (val, font) = if val == "" {
            match &mut self.config.placeholder {
                Some(v) => (v.text.as_str(), &mut v.font),
                None => ("", &mut self.config.font),
            }
        } else {
            (val.as_str(), &mut self.config.font)
        };

        let (size_before_cursor, total_length, glyphs) = Self::get_glyphs(
            self.cursor_at_from_left,
            gfx,
            val,
            font,
            self.config.location.x(),
        )?;
        
        gfx.flush_window(window).expect("could not flush");
        let mut surface = Surface::new(
            gfx,
            Image::from_raw(gfx, None, 512, 512, PixelFormat::RGBA)?,
        )?;
        gfx.fit_to_surface(&surface)?;
        gfx.clear(Color::BLUE);
        gfx.fill_rect(&Rectangle::new((0., 0.).into(), (10., 10.).into()), Color::GREEN);
        gfx.flush_surface(&surface)?;
        //gfx.clear(Color::RED);
        gfx.fit_to_window(&window);
        let image = surface
            .detach()
            .ok_or(quicksilver::QuicksilverError::SurfaceImageError)?;
        gfx.draw_image(&image, Rectangle::new((10., 10.).into(), (512., 512.).into()));

        /*
        let offset = Self::calc_offset(
            self.config.location.width(),
            total_length,
            size_before_cursor,
        );
        glyphs.iter().enumerate().for_each(|(key, layout_glyph)| {
            let glyph_bounds = layout_glyph.glyph.bounds;
            let pos = Vector::new(
                layout_glyph.position.x - offset,
                layout_glyph.position.y as f32 + self.config.font.font.size,
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
            if key + 1 == self.cursor_at_from_left {
                let cursor_pos = Vector::new(pos.x + glyph_bounds.width as f32, 0);
                gfx.fill_rect(
                    &Rectangle::new(cursor_pos, Vector::new(1, self.config.location.height())),
                    Color::GREEN,
                );
            }
        });*/
        /*
        if self.cursor_at_from_left == 0 {
            gfx.fill_rect(
                &Rectangle::new(
                    Vector::new(0.0, 0.0),
                    Vector::new(1, self.config.location.height()),
                ),
                Color::GREEN,
            );
        }*/

        //gfx.draw_image(&image, pos);

        /*
        let glyphs_to_draw: Vec<_> =
            self.select_glyphs_to_draw(total_length, length_before_cursor, glyphs);

        let mut cursor_x = self.config.location.pos.x;
        let x_offset = glyphs_to_draw
            .get(0)
            .map(|(_, val)| val.position.x as f32)
            .unwrap_or_default()
            - self.config.location.x();

        let mut last_x = -1.0;
        let mut cursor_key = 0;
        glyphs_to_draw
            .into_iter()
            .map(|(key, glyph)| (key + 1, glyph))
            .for_each(|(_, glyph)| {
                let glyph_bounds = glyph.glyph.bounds;
                let pos = Vector::new(
                    glyph.position.x - x_offset,
                    glyph.position.y as f32 + self.config.location.y() + self.config.font.font.size,
                );

                let glyph_size = Vector::new(glyph_bounds.width as f32, glyph_bounds.height as f32);
                let region = Rectangle::new(
                    Vector::new(glyph_bounds.x as f32, glyph_bounds.y as f32),
                    glyph_size,
                );

                gfx.draw_subimage_tinted(
                    &glyph.image,
                    region,
                    Rectangle::new(pos, glyph_size),
                    self.config.font.color,
                );
                last_x = pos.x + glyph_bounds.width as f32;
                cursor_key += 1;
                if cursor_key == self.cursor_at_from_left {
                    cursor_x = last_x;
                }
            });

        if cursor_key < self.cursor_at_from_left {
            cursor_x = last_x;
        }

        gfx.fill_rect(
            &Rectangle::new(
                Vector::new(cursor_x, self.config.location.y()),
                Vector::new(1, self.config.location.height()),
            ),
            Color::GREEN,
        );

        Ok(())*/
        Ok(())
    }
}

impl Widget for Input {
    fn contains(&self, pos: &Vector) -> bool {
        self.config.location.contains((pos.x, pos.y).into())
    }
    fn is_focusable(&self, _: &Vector) -> bool {
        true
    }
    fn render(&mut self, gfx: &mut Graphics, window: &Window) -> Result<()> {
        //gfx.stroke_rect(&self.config.location, Color::BLACK);
        self.draw_text(gfx, window)
    }

    fn get_cursor_on_hover(&self, _: &Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Text
    }

    fn on_key_press(&mut self, key: quicksilver::input::Key, state: bool) {
        use quicksilver::input::Key::*;
        if Back == key && state && self.cursor_at_from_left > 0 {
            self.value.remove_char_at(self.cursor_at_from_left - 1);
            self.cursor_at_from_left -= 1;
        }
        if key == Left && state && self.cursor_at_from_left > 0 {
            self.cursor_at_from_left -= 1;
        }
        if key == Right && state {
            let size = self.value.char_count();
            if self.cursor_at_from_left < size {
                self.cursor_at_from_left += 1;
            } else {
                println!("reached end. key : {}", size);
            }
        }
    }

    fn on_typed(&mut self, typed_char: char) {
        if typed_char.is_control() {
            return;
        }
        if self.cursor_at_from_left > self.value.char_count() {
            self.value.push(typed_char)
        } else {
            self.value
                .insert_char_at_place(self.cursor_at_from_left, typed_char)
        };
        self.cursor_at_from_left += 1;
    }
}
