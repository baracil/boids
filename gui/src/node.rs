use std::rc::Rc;

use raylib::color::Color;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;
use raylib::text::{Font, measure_text_ex};

use crate::alignment::Alignment;
use crate::mouse::MouseState;

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const CONTENT_SIZE = 2;
        const SIZE = 4;
        const POSITION = 8;

        const ALL = 15;
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width:f32,
    pub height:f32,
}

impl Size {
    pub fn empty() -> Self {
        Self{width:0.0,height:0.0}
    }
}


/// A background
pub trait Background {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}

/// A border
pub trait Border {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}

/// for a node that has children
pub trait Parent {
    fn children(&self) -> &[Box<dyn Node>];
}

pub trait Node {
    fn parent(&self) -> &Option<Rc<dyn Node>>;

    /// the width of the content (computed)
    fn content_width(&self) -> f32;
    fn content_height(&self) -> f32;

    /// padding around the content
    fn padding(&self) -> f32;

    fn content_width_with_padding(&self) -> f32 {
        self.content_width()+self.padding()
    }
    fn content_height_with_padding(&self) -> f32 {
        self.content_height()+self.padding()
    }

    /// set the position of this node
    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn Node;

    fn set_padding(&mut self, padding:f32) -> &mut dyn Node;

    fn clear_requested_size(&mut self) -> &mut dyn Node;
    fn set_requested_height(&mut self, height:f32) -> &mut dyn Node;
    fn set_requested_width(&mut self, width:f32) -> &mut dyn Node;
    fn set_requested_size(&mut self, size:Size) -> &mut dyn Node;
}

pub trait LayoutableNode {
    /// compute size and position of the node
    fn layout(&mut self);
}

pub trait UpdatableNode {
    /// update state (armed, hoover, clicked) with current mouse position and mouse button states
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
}

pub trait RenderableNode {
    /// draw the node
    fn render(&self, d: &mut RaylibDrawHandle);
}


