
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use crate::button::ButtonPar;
use crate::mouse::MouseState;

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const COMPUTED_SIZE = 2;
        const SIZE = 4;
        const POSITION = 8;
        const ALL = 15;
    }
}


pub trait Background {
    fn draw(&self, d: &mut RaylibDrawHandle, geometry: &Rectangle);
}

pub trait Border {
    fn draw(&self, d: &mut RaylibDrawHandle, geometry: &Rectangle);
}

pub trait Parent {
    fn children(&self) -> &[Box<dyn Node>];
}

pub trait Node : NodePar {
    fn parent(&self) -> &Option<Box<dyn Node>>;
    fn item(&self) -> &Item;
}


pub trait NodePar {
    fn content_width(&self) -> f32;
    fn content_height(&self) -> f32;
    fn padding(&self) -> f32;

    fn content_width_with_padding(&self) -> f32 {
        self.content_width()+self.padding()
    }

    fn layout(&mut self);
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
    fn render(&self, d: &mut RaylibDrawHandle);

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn NodePar;
    fn set_padding(&mut self, padding:f32) -> &mut dyn NodePar;

}

#[derive(Copy, Clone)]
pub struct Size {
    pub width:f32,
    pub height:f32,
}

impl Size {
    pub fn new() -> Self {
        Self{width:0.0,height:0.0}
    }
}

pub enum Item {
    Button(ButtonPar),
}



