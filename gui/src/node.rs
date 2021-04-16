
use raylib::prelude::*;
use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use crate::button::ButtonPar;
use crate::mouse::MouseState;
use crate::label::LabelPar;
use raylib::text::{Font, measure_text_ex};
use raylib::color::Color;
use crate::private_node::InnerNode;
use std::rc::Rc;

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const CONTENT_SIZE = 2;
        const SIZE = 4;
        const POSITION = 8;

        const ALL = 15;
    }
}

#[derive(Clone)]
pub struct FontInfo {
    pub font:Rc<Font>,
    pub size:i32
}

impl FontInfo {
    pub fn measure_text(&self, text:&str, spacing:f32) -> Size {
        let size = measure_text_ex(&self.font.as_ref(),text,self.size as f32 ,spacing);
        Size {width:size.x, height:size.y}
    }

    pub fn draw_text(&self, d:&mut RaylibDrawHandle<'_>, text:&str, position:&Vector2, spacing:f32, color:Color) {
        d.draw_text_ex(&self.font.as_ref(),text, position,self.size as f32,spacing,color)
    }
}



// Draw a background
pub trait Background {
    fn draw(&self, d: &mut RaylibDrawHandle, geometry: &Rectangle);
}

// Draw a border
pub trait Border {
    fn draw(&self, d: &mut RaylibDrawHandle, geometry: &Rectangle);
}

// for a node that has children
pub trait Parent {
    fn children(&self) -> &[Box<dyn Node>];
}



pub trait NodePar {

    //the width of the content (computed)
    fn content_width(&self) -> f32;
    fn content_height(&self) -> f32;
    //padding around the content
    fn padding(&self) -> f32;

    fn content_width_with_padding(&self) -> f32 {
        self.content_width()+self.padding()
    }
    fn content_height_with_padding(&self) -> f32 {
        self.content_height()+self.padding()
    }

    //compute size and position of the node
    fn layout(&mut self);
    //update state (armed, hoover, clicked) with current mouse position and mouse button states
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
    //draw the node
    fn render(&self, d: &mut RaylibDrawHandle);

    // set the position of this node
    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn NodePar;

    fn set_padding(&mut self, padding:f32) -> &mut dyn NodePar;

    fn clear_requested_size(&mut self) -> &mut dyn NodePar;
    fn set_requested_height(&mut self, height:f32) -> &mut dyn NodePar;
    fn set_requested_width(&mut self, width:f32) -> &mut dyn NodePar;
    fn set_requested_size(&mut self, size:Size) -> &mut dyn NodePar;
}

pub trait Node : NodePar {
    fn parent(&self) -> &Option<Box<dyn Node>>;
    fn item(&self) -> &Item;
    fn item_mut(&mut self) -> &mut Item;
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

    pub fn none() -> Self {
        Self{width:-1.0,height:-1.0}
    }
}

pub enum Item {
    Button(ButtonPar),
    Label(LabelPar),
}


