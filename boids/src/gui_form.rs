use std::ops::DerefMut;

use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};

pub enum GuiElement {
    Button(Box<ButtonPar>),
    Slider(Box<SlidePar>),
}

pub fn get_gui_item(element: &mut GuiElement) -> &mut dyn GuiItem {
    match element {
        GuiElement::Button(a) => a.deref_mut(),
        GuiElement::Slider(a) => a.deref_mut(),
    }
}

pub trait GuiItem {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn geometry(&self) -> Rectangle;
    fn color(&self) -> Color;
    fn set_position(&mut self, position: &Vector2);
    fn background_color(&self) -> Color;
}

pub trait Drawable {
    fn draw(&self, rs: &mut RaylibDrawHandle, position: &Vector2);
}

impl GuiItem for ButtonPar {
    fn width(&self) -> f32 {
        self.geometry.width
    }

    fn height(&self) -> f32 {
        self.geometry.height
    }

    fn geometry(&self) -> Rectangle {
        self.geometry
    }

    fn color(&self) -> Color {
        self.color
    }

    fn set_position(&mut self, position: &Vector2) {
        self.geometry.y = position.y;
        self.geometry.x = position.x;
    }

    fn background_color(&self) -> Color {
        return self.background_color;
    }
}
impl GuiItem for SlidePar {
    fn width(&self) -> f32 {
        self.geometry.width
    }

    fn height(&self) -> f32 {
        self.geometry.height
    }

    fn geometry(&self) -> Rectangle {
        return self.geometry;
    }
    fn color(&self) -> Color {
        self.color
    }

    fn set_position(&mut self, position: &Vector2) {
        self.geometry.y = position.y;
        self.geometry.x = position.x;
    }

    fn background_color(&self) -> Color {
        return self.background_color;
    }
}

pub struct ButtonPar {
    pub id: String,
    pub geometry: Rectangle,
    pub color: Color,
    pub background_color: Color,
    pub text: String,
}

pub struct SlidePar {
    pub id: String,
    pub geometry: Rectangle,
    pub color: Color,
    pub background_color: Color,
    pub value_min: f32,
    pub value_max: f32,
}
