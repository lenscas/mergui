use crate::widgets::{widget_traits::WidgetConfig, Widget};
use crate::{channels::InputChannel, FontStyle};
use quicksilver::geom::{Rectangle, Shape};
use quicksilver::{graphics::Color, mint::Vector2};

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
}

impl WidgetConfig<InputChannel, Input> for InputConfig {
    fn to_widget(self) -> (Input, InputChannel) {
        let value = self
            .start_value
            .clone()
            .map(|v| v)
            .unwrap_or("".into())
            .into();
        (
            Input {
                config: self,
                value: InputChannel::clone(&value),
            },
            value,
        )
    }
}

impl Widget for Input {
    fn contains(&self, pos: &Vector2<f32>) -> bool {
        self.config.location.contains((pos.x, pos.y))
    }
    fn is_focusable(&self, _: &Vector2<f32>) -> bool {
        true
    }
    fn render(&mut self, gfx: &mut quicksilver::graphics::Graphics) {
        gfx.stroke_rect(&self.config.location, Color::BLACK);
        let val = self.value.get();
        let (val, font) = if val == "" {
            match &mut self.config.placeholder {
                Some(v) => (v.text.as_str(), &mut v.font),
                None => ("", &mut self.config.font),
            }
        } else {
            (val.as_str(), &mut self.config.font)
        };
        let pos = {
            let mut pos = self.config.location.pos;
            pos.y += font.size;
            pos
        };
        let mut v = font.font.0.borrow_mut();
        gfx.draw_text(&mut v, &val, font.size, font.max_width, font.color, pos);
    }
    fn get_cursor_on_hover(&self, _: &Vector2<f32>) -> quicksilver::lifecycle::CursorIcon {
        quicksilver::lifecycle::CursorIcon::Text
    }

    fn on_key_press(&mut self, key: quicksilver::lifecycle::Key, state: bool) {
        if quicksilver::lifecycle::Key::Back == key && state {
            let mut old = self.value.get();
            old.pop();
            self.value.set(old);
        }
    }

    fn on_typed(&mut self, typed_char: char) {
        if typed_char.is_control() {
            return;
        }
        let mut old = self.value.get();
        old.push(typed_char);
        self.value.set(old);
    }
}
