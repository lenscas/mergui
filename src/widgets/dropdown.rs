use super::{Widget, WidgetConfig};
use crate::{channels::Dropdown as Channel, force_mutex, Assets};
use quicksilver::prelude::{
    Background::Col, Color, Image, Img, Line, Rectangle, Shape, Transform, Vector, Window,
};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, MutexGuard},
};

///used to configure every value that a dropdown contains.
pub struct DropDownValueConfig<T: Clone> {
    ///The actual value that is being represented
    pub value: T,
    ///what is displayed in the list
    pub normal: Image,
    ///what is displayed if the user hovers over it
    pub hover: Option<Image>,
}
impl<T: Clone> From<(T, Image)> for DropDownValueConfig<T> {
    fn from(val: (T, Image)) -> DropDownValueConfig<T> {
        Self {
            value: val.0,
            normal: val.1,
            hover: None,
        }
    }
}
impl<T: Clone> From<(T, Image, Image)> for DropDownValueConfig<T> {
    fn from(val: (T, Image, Image)) -> DropDownValueConfig<T> {
        Self {
            value: val.0,
            normal: val.1,
            hover: Some(val.2),
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
    pub open_button: String,
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
    pub values: Arc<Mutex<Vec<DropDownValueConfig<T>>>>,
    pub is_open: Arc<Mutex<bool>>,
    pub selected: Arc<Mutex<Option<usize>>>,
    pub open_button: String,
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
        let is_open = Arc::new(Mutex::new(false));
        let selected = Arc::new(Mutex::new(self.selected));
        let values = Arc::new(Mutex::new(values));
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
    fn contains(&self, point: &Vector) -> bool {
        self.location.contains(*point)
            || (self.is_open() && self.get_open_rec().contains(*point))
            || self.get_location_open_button().contains(*point)
    }
    fn is_focusable(&self, _: &Vector) -> bool {
        true
    }
    fn set_hover(&mut self, point: &Vector, state: bool) {
        if state {
            self.hover_over = Some(point.clone());
        } else {
            self.hover_over = None;
        }
    }
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32) {
        window.draw_ex(
            &self.get_location_open_button(),
            Img(assets.get_image(&self.open_button)),
            Transform::IDENTITY,
            z,
        );
        self.draw_arround_rec(&self.location, window, z);
        let values = self.values();
        let selected = self
            .selected()
            .and_then(|v| values.get(v))
            .or_else(|| values.get(0));

        if let Some(selected) = selected {
            window.draw_ex(
                &self.location,
                Img(&selected.normal),
                Transform::IDENTITY,
                z + 1,
            );
        }
        drop(values);
        let hovered = self.hover_over.and_then(|v| self.vector_to_index(&v));

        if self.is_open() {
            self.values()
                .iter()
                .enumerate()
                .map(|(key, value)| match hovered {
                    Some(x) => {
                        if x == key {
                            (value.hover.as_ref().unwrap_or(&value.normal), key)
                        } else {
                            (&value.normal, key)
                        }
                    }
                    None => (&value.normal, key),
                })
                .map(|(image, key)| (image, (key + 1) as f32))
                .map(|(img, index)| {
                    let mut loc = self.location.clone();
                    loc.pos.y = loc.pos.y + (self.option_height * index);
                    (img, loc)
                })
                .for_each(|(img, location)| {
                    window.draw_ex(&location, Img(img), Transform::IDENTITY, z + 1);
                    self.draw_arround_rec(&location, window, z + 1);
                })
        }
    }
    fn on_click(&mut self, pos: &Vector) {
        if let Some(selected) = self.vector_to_index(&pos) {
            *force_mutex(&self.selected) = Some(selected);
        }
        let mut open = force_mutex(&self.is_open);
        *open = !*open;
    }
    fn get_cursor_on_hover(&self, _: &Vector) -> quicksilver::input::MouseCursor {
        quicksilver::input::MouseCursor::Hand
    }
    fn set_focus(&mut self, _: &Vector, focus: bool) {
        if focus == false {
            *force_mutex(&self.is_open) = focus;
        }
    }
}
impl<T: Clone> DropDown<T> {
    pub fn get_location_open_button(&self) -> Rectangle {
        let mut open_button_location = self.location.clone();
        open_button_location.size = self.open_button_size.clone();
        open_button_location.pos.x = self.location.pos.x + self.location.width();
        open_button_location
    }
    pub fn get_open_rec(&self) -> Rectangle {
        let mut rec = self.location.clone();
        rec.size.y = self.location.size.y + (self.option_height * self.values().len() as f32);
        rec
    }
    pub fn is_open(&self) -> bool {
        *force_mutex(&self.is_open)
    }
    pub fn values(&self) -> MutexGuard<Vec<DropDownValueConfig<T>>> {
        force_mutex(&self.values)
    }
    pub fn selected(&self) -> Option<usize> {
        *force_mutex(&self.selected)
    }
    pub fn vector_to_index(&self, point: &Vector) -> Option<usize> {
        if !self.is_open() {
            return None;
        }
        let mut offset = point.clone();
        offset.y -= self.location.pos.y + self.location.height();
        let offset = offset;
        if offset.y > 0f32 {
            let size = self.values().len() as f32 * self.option_height;
            let selected = (self.values().len() as f32 - ((size - offset.y) / self.option_height))
                .floor() as usize;
            Some(selected)
        } else {
            None
        }
    }
    fn draw_arround_rec(&self, rec: &Rectangle, window: &mut Window, z: u32) {
        let corners = vec![
            rec.pos,
            {
                let mut v = rec.pos;
                v.y += rec.height();
                v
            },
            {
                let mut v = rec.pos;
                v.y += rec.height();
                v.x += rec.width();
                v
            },
            {
                let mut v = rec.pos;
                v.x += rec.width();
                v
            },
        ];
        let mut combined: Vec<(Vector, Vector)> = Vec::new();
        for (index, value) in corners.iter().enumerate() {
            combined.push((
                value.clone(),
                corners
                    .get(index + 1)
                    .cloned()
                    .unwrap_or_else(|| corners[0]),
            ));
        }
        combined.into_iter().for_each(|(start, end)| {
            window.draw_ex(
                &Line::new(start, end).with_thickness(self.divider_size),
                Col(self.divider_color),
                Transform::IDENTITY,
                z + 1,
            )
        });
    }
}
