use super::{Widget, WidgetConfig};
use crate::{channels::Dropdown as Channel, FontStyle};
use quicksilver::geom::Rectangle;
use quicksilver::geom::Shape;
use quicksilver::geom::Vector;
use quicksilver::graphics::Color;
use quicksilver::graphics::Graphics;
use quicksilver::graphics::Image;
use quicksilver::{Result, Window};

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

///used to configure every value that a dropdown contains.
pub struct DropDownValueConfig<T: Clone> {
    ///The actual value that is being represented
    pub value: T,
    ///what is displayed in the list
    pub text: String,
    pub normal_font_style: FontStyle,
    ///what is displayed if the user hovers over it
    pub hover_font_style: Option<FontStyle>,
}
impl<T: Clone> From<(T, String, FontStyle)> for DropDownValueConfig<T> {
    fn from(val: (T, String, FontStyle)) -> DropDownValueConfig<T> {
        Self {
            value: val.0,
            text: val.1,
            normal_font_style: val.2,
            hover_font_style: None,
        }
    }
}
impl<T: Clone> From<(T, String, FontStyle, FontStyle)> for DropDownValueConfig<T> {
    fn from(val: (T, String, FontStyle, FontStyle)) -> DropDownValueConfig<T> {
        Self {
            value: val.0,
            text: val.1,
            normal_font_style: val.2,
            hover_font_style: Some(val.3),
        }
    }
}
impl<T: Clone + ToString> From<(T, FontStyle, FontStyle)> for DropDownValueConfig<T> {
    fn from(val: (T, FontStyle, FontStyle)) -> DropDownValueConfig<T> {
        let as_text = val.0.to_string();
        Self {
            value: val.0,
            text: as_text,
            normal_font_style: val.1,
            hover_font_style: Some(val.2),
        }
    }
}
impl<T: Clone + ToString> From<(T, FontStyle)> for DropDownValueConfig<T> {
    fn from(val: (T, FontStyle)) -> DropDownValueConfig<T> {
        let as_text = val.0.to_string();
        Self {
            value: val.0,
            text: as_text,
            normal_font_style: val.1,
            hover_font_style: None,
        }
    }
}
///Configures the dropdown widget
pub struct DropDownConfig<T: Clone, ValueConfig: Into<DropDownValueConfig<T>>> {
    ///A list of selectable values and how to render them. See DropDownValueConfig
    pub values: Vec<ValueConfig>,
    ///The location of this widget
    pub location: Rectangle,
    ///The height of every option inside the list
    pub option_height: f32,
    ///The image that is used to show an extra button the user can click on to open it
    pub open_button: Image,
    ///The size of the button. The button itself is always left to the widget
    pub open_button_size: Vector,
    ///What starts as selected
    pub selected: Option<usize>,
    ///The color of the line between and arround every option
    pub divider_color: Color,
    ///The width of the line between and arround every option
    pub divider_size: f32,
    pub t: PhantomData<T>,
}

pub struct DropDown<T: Clone> {
    pub location: Rectangle,
    pub option_height: f32,
    pub values: Rc<RefCell<Vec<DropDownValueConfig<T>>>>,
    pub is_open: Rc<RefCell<bool>>,
    pub selected: Rc<RefCell<Option<usize>>>,
    pub open_button: Image,
    pub open_button_size: Vector,
    pub hover_over: Option<Vector>,
    pub divider_color: Color,
    pub divider_size: f32,
}

impl<T: Clone, X: Into<DropDownValueConfig<T>>> WidgetConfig<Channel<T>, DropDown<T>>
    for DropDownConfig<T, X>
{
    fn to_widget(self) -> (DropDown<T>, Channel<T>) {
        let values = self.values.into_iter().map(Into::into).collect();
        let is_open = Rc::new(RefCell::new(false));
        let selected = Rc::new(RefCell::new(self.selected));
        let values = Rc::new(RefCell::new(values));
        let channel = Channel {
            values: values.clone(),
            is_open: is_open.clone(),
            selected: selected.clone(),
        };

        (
            DropDown {
                location: self.location,
                option_height: self.option_height,
                open_button: self.open_button,
                open_button_size: self.open_button_size,
                divider_size: self.divider_size,
                divider_color: self.divider_color,
                values,
                is_open,
                selected,
                hover_over: None,
            },
            channel,
        )
    }
}

impl<T: Clone> Widget for DropDown<T> {
    fn contains(&self, point: Vector) -> bool {
        self.location.contains(point)
            || (self.is_open() && self.get_open_rec().contains(point))
            || self.get_location_open_button().contains(point)
    }
    fn is_focusable(&self, _: Vector) -> bool {
        true
    }
    fn set_hover(&mut self, point: Vector, state: bool) {
        if state {
            self.hover_over = Some(point);
        } else {
            self.hover_over = None;
        }
    }
    fn render(&mut self, gfx: &mut Graphics, _: &Window) -> Result<()> {
        gfx.draw_image(&self.open_button, self.get_location_open_button());
        self.draw_arround_rec(&self.location, gfx);
        let values = self.values.borrow();
        let selected = self
            .selected()
            .and_then(|v| values.get(v))
            .or_else(|| values.get(0));

        if let Some(selected) = selected {
            let mut pos = self.location.pos;
            pos.y += selected.normal_font_style.font.size;
            selected.normal_font_style.font.draw(
                gfx,
                &selected.text,
                selected.normal_font_style.color,
                pos,
            )?;
        }
        drop(values);
        let hovered = self.hover_over.and_then(|v| self.vector_to_index(v));

        if self.is_open() {
            self.values
                .borrow_mut()
                .iter_mut()
                .enumerate()
                .map(|(key, value)| match hovered {
                    Some(x) => {
                        if x == key {
                            (
                                &value.text,
                                value
                                    .hover_font_style
                                    .as_ref()
                                    .unwrap_or(&value.normal_font_style),
                                key,
                            )
                        } else {
                            (&value.text, &value.normal_font_style, key)
                        }
                    }
                    None => (&value.text, &value.normal_font_style, key),
                })
                .map(|(text, font, key)| (text, font, (key + 1) as f32))
                .map(|(text, font, index)| {
                    let mut loc_box = self.location;
                    let mut loc_text = self.location;
                    loc_box.pos.y += self.option_height * index;
                    loc_text.pos.y += (self.option_height * index) + font.font.size;
                    (text, font, loc_box, loc_text)
                })
                .map(|(text, font_style, location_box, location_text)| {
                    font_style
                        .font
                        .draw(gfx, text, font_style.color, location_text.pos)?;
                    self.draw_arround_rec(&location_box, gfx);
                    Ok(())
                })
                .collect::<Result<_>>()?;
        }
        Ok(())
    }
    fn on_click(&mut self, pos: Vector) {
        if let Some(selected) = self.vector_to_index(pos) {
            self.selected.swap(&RefCell::new(Some(selected)));
        }

        let open = *self.is_open.borrow();
        self.is_open.swap(&RefCell::new(!open));
    }
    fn get_cursor_on_hover(&self, _: Vector) -> quicksilver::CursorIcon {
        quicksilver::CursorIcon::Hand
    }
    fn set_focus(&mut self, _: Vector, focus: bool) {
        if !focus {
            self.is_open.swap(&RefCell::new(focus));
        }
    }
}
impl<T: Clone> DropDown<T> {
    pub fn get_location_open_button(&self) -> Rectangle {
        let mut open_button_location = self.location;
        open_button_location.size = Vector::new(self.open_button_size.x, self.open_button_size.y);
        open_button_location.pos.x = self.location.pos.x + self.location.width();
        open_button_location
    }
    pub fn get_open_rec(&self) -> Rectangle {
        let mut rec = self.location;
        rec.size.y =
            self.location.size.y + (self.option_height * self.values.borrow().len() as f32);
        rec
    }
    pub fn is_open(&self) -> bool {
        *self.is_open.borrow()
    }
    pub fn selected(&self) -> Option<usize> {
        *self.selected.borrow()
    }

    pub fn vector_to_index(&self, point: Vector) -> Option<usize> {
        if !self.is_open() {
            return None;
        }
        let mut offset = point;
        offset.y -= self.location.pos.y + self.location.height();
        let offset = offset;
        if offset.y > 0f32 {
            let len = self.values.borrow().len();
            let size = len as f32 * self.option_height;
            let selected = (len as f32 - ((size - offset.y) / self.option_height)).floor() as usize;
            Some(selected)
        } else {
            None
        }
    }
    fn draw_arround_rec(&self, rec: &Rectangle, gfx: &mut Graphics) {
        gfx.stroke_rect(rec, self.divider_color);
    }
}
