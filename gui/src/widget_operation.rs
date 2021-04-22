
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::*;

use crate::alignment::{Alignment, VAlignment, HAlignment};
use crate::mouse::MouseState;
use crate::gui::{Gui, GuiData};
use crate::padding::Padding;
use crate::size::Size;
use crate::fill::Fill;

bitflags! {
    pub struct DirtyFlags: u32 {
        const PREFERRED_SIZE = 2;
        const CONTENT_SIZE = 4;
        const SIZE = 8;
        const POSITION = 16;

        const ALL = 31;
    }
}

pub trait WidgetOp {
    /// the width of the content (computed)
    fn content_width(&self) -> f32;
    fn content_height(&self) -> f32;

    /// padding around the content
    fn padding(&self) -> Padding;

    /// set the position of this node
    fn set_position_vec(&self,gui:&Gui, point: &Vector2, valignment: VAlignment, halignment: HAlignment) -> &dyn WidgetOp  {
        self.set_position(gui,point.x,point.y);
        self.set_valignment(gui,valignment);
        self.set_halignment(gui,halignment)
    }

    fn set_position_ex(&self,gui:&Gui, x: f32, y:f32, valignment: VAlignment, halignment: HAlignment) -> &dyn WidgetOp  {
        self.set_position(gui,x,y);
        self.set_valignment(gui,valignment);
        self.set_halignment(gui,halignment)
    }

    fn set_absolute_coordinate_y(&self,gui:&Gui,absolute:bool)  -> &dyn WidgetOp ;
    fn set_absolute_coordinate_x(&self,gui:&Gui,absolute:bool)  -> &dyn WidgetOp ;

    fn set_position(&self,gui:&Gui, x: f32, y:f32)  -> &dyn WidgetOp;
    fn set_valignment(&self,gui:&Gui, valignment: VAlignment)  -> &dyn WidgetOp;
    fn set_halignment(&self,gui:&Gui, halignment: HAlignment)  -> &dyn WidgetOp;

    fn set_padding(&self,gui:&Gui, padding: Padding)  -> &dyn WidgetOp;

    fn clear_requested_size(&self,gui:&Gui)  -> &dyn WidgetOp;
    fn set_requested_height(&self,gui:&Gui, height: f32)  -> &dyn WidgetOp;
    fn set_requested_width(&self,gui:&Gui, width: f32)  -> &dyn WidgetOp;
    fn set_requested_size(&self,gui:&Gui, size: Size)  -> &dyn WidgetOp;

    fn disable_fill_width(&self,gui:&Gui) -> &dyn WidgetOp;
    fn disable_fill_height(&self,gui:&Gui) -> &dyn WidgetOp;

    fn enable_fill_width(&self,gui:&Gui, fill:Fill) -> &dyn WidgetOp;
    fn enable_fill_height(&self,gui:&Gui, fill:Fill) -> &dyn WidgetOp;

}


pub trait UpdatableWidget {
    /// update state (armed, hoover, clicked) with current mouse position and mouse button states
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
}

pub trait RenderableWidget {
    /// draw the node
    fn render(&self, gui:&Gui, d: &mut RaylibDrawHandle, position:Vector2);
}
