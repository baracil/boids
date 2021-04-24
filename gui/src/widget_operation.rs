use raylib::prelude::*;

use crate::alignment::{HAlignment, VAlignment};
use crate::fill::Fill;
use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::padding::Padding;
use crate::size::Size;
use crate::widget_data::WidgetData;

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const PREFERRED_SIZE = 2;
        const CONTENT_SIZE = 4;
        const POSITION = 8;

        const ALL = 15;
    }
}

pub trait WidgetOp {

    fn set_text_style(&self, text_style_name:&str) -> &dyn WidgetOp;
    fn set_background_style(&self, background_style_name:&str) -> &dyn WidgetOp;

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

    fn clear_preferred_size(&self,gui:&Gui)  -> &dyn WidgetOp {
        self.set_preferred_size(gui,Size::empty())
    }

    fn set_preferred_height(&self,gui:&Gui, height: f32)  -> &dyn WidgetOp;
    fn set_preferred_width(&self,gui:&Gui, width: f32)  -> &dyn WidgetOp;
    fn set_preferred_size(&self,gui:&Gui, size: Size)  -> &dyn WidgetOp;

    fn disable_fill_width(&self,gui:&Gui) -> &dyn WidgetOp;
    fn disable_fill_height(&self,gui:&Gui) -> &dyn WidgetOp;

    fn enable_fill_width(&self,gui:&Gui, fill:Fill) -> &dyn WidgetOp;
    fn enable_fill_height(&self,gui:&Gui, fill:Fill) -> &dyn WidgetOp;

}

pub trait SizeComputer {
    fn compute_size(&self, gui:&Gui) -> Size;
    fn compute_child_content_size(&self, gui:&Gui, available_size:Size);
    fn compute_child_positions(&self, gui:&Gui);
}

pub trait LayoutableWidget {
    fn get_computed_size(&self, gui: &Gui) -> Size;
    fn update_content_size(&self, gui: &Gui, available_space: &Size);
    fn update_child_positions(&self, gui:&Gui);
}

pub trait UpdatableWidget {
    /// update state (armed, hoover, clicked) with current mouse position and mouse button states
    fn update_with_mouse_information(&mut self, mouse_position: &Vector2, mouse_state: &MouseState);
}

pub trait RenderableWidget {
    /// draw the node
    fn render(&self, gui:&Gui, d: &mut RaylibDrawHandle, offset:&Vector2, available_size:&Size);
}

pub trait WidgetDataProvider {
    fn widget_data(&self) -> &WidgetData;
    fn widget_data_mut(&mut self) -> &mut WidgetData;
}
