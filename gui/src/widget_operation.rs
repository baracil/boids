
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;

use crate::alignment::{Alignment, VAlignment, HAlignment};
use crate::mouse::MouseState;
use crate::gui::{Gui, GuiData};

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
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn empty() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
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

pub trait WidgetOp {
    /// the width of the content (computed)
    fn content_width(&self) -> f32;
    fn content_height(&self) -> f32;

    /// padding around the content
    fn padding(&self) -> f32;

    fn content_width_with_padding(&self) -> f32 {
        self.content_width() + self.padding()
    }
    fn content_height_with_padding(&self) -> f32 {
        self.content_height() + self.padding()
    }

    /// set the position of this node
    fn set_position_vec(&mut self, point: &Vector2, valignment: VAlignment, halignment: HAlignment) -> &mut dyn WidgetOp {
        self.set_position(point.x,point.y);
        self.set_valignment(valignment);
        self.set_halignment(halignment)
    }

    fn set_position_ex(&mut self, x: f32, y:f32, valignment: VAlignment, halignment: HAlignment) -> &mut dyn WidgetOp {
        self.set_position(x,y);
        self.set_valignment(valignment);
        self.set_halignment(halignment)
    }

    fn set_absolute_coordinate(&mut self,absolute:bool) -> &mut dyn WidgetOp;

    fn set_position(&mut self, x: f32, y:f32) -> &mut dyn WidgetOp;
    fn set_valignment(&mut self, valignment: VAlignment) -> &mut dyn WidgetOp;
    fn set_halignment(&mut self, halignment: HAlignment) -> &mut dyn WidgetOp;

    fn set_padding(&mut self, padding: f32) -> &mut dyn WidgetOp;

    fn clear_requested_size(&mut self) -> &mut dyn WidgetOp;
    fn set_requested_height(&mut self, height: f32) -> &mut dyn WidgetOp;
    fn set_requested_width(&mut self, width: f32) -> &mut dyn WidgetOp;
    fn set_requested_size(&mut self, size: Size) -> &mut dyn WidgetOp;

    fn set_fill_width(&mut self, fill:bool) -> &mut dyn WidgetOp;
    fn set_fill_height(&mut self, fill:bool) -> &mut dyn WidgetOp;

    fn set_fill_width_weight(&mut self, weight:u32) -> &mut dyn WidgetOp;
    fn set_fill_height_weight(&mut self, weight:u32) -> &mut dyn WidgetOp;

}

pub trait LayoutableWidget {
    /// compute size and position of the node
    fn layout(&mut self, available_space:&Size);
}

pub trait UpdatableWidget {
    /// update state (armed, hoover, clicked) with current mouse position and mouse button states
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
}

pub trait RenderableWidget {
    /// draw the node
    fn render(&self, gui_data:&GuiData, d: &mut RaylibDrawHandle);
}
