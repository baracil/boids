use raylib::prelude::*;

use crate::alignment::{HAlignment, VAlignment};
use crate::fill::Fill;
use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::padding::Padding;
use crate::size::Size;
use crate::widget_data::WidgetData;
use crate::position::{Coordinate, Position};

bitflags! {
    pub struct DirtyFlags: u32 {
        const STYLE = 1;
        const PREFERRED_SIZE = 2;
        const CONTENT_SIZE = 4;
        const POSITION = 8;

        const ALL = 15;
    }
}

pub trait WidgetSpecific {
    fn get_widget_data(&self) -> &WidgetData;
    fn get_widget_data_mut(&mut self) -> &mut WidgetData;
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
    fn update_with_mouse_information(&self, gui:&Gui, offset:&Vector2, mouse_position: &Vector2, mouse_state: &MouseState);
}

pub trait RenderableWidget {
    /// draw the widget
    fn render(&self, gui:&Gui, d: &mut impl RaylibDraw, offset:&Vector2);
}

