use std::rc::Rc;
use std::cell::RefCell;
use crate::widget::Widget;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};
use crate::widget_operation::{RenderableWidget, Size};
use raylib::prelude::*;
use crate::gui::{GuiData, RefGuiData};

pub struct PanePar {
    widget_data: WidgetData,

}

impl PanePar {
    pub fn new(gui_data:RefGuiData) -> Self {
        Self{widget_data:WidgetData::new(gui_data)}
    }
}

impl SizeableWidget for PanePar {
    fn compute_content_size(&self, available_size: &Size) -> Size {
        let mut width = self.widget_data.geometry.requested_size.width;
        let mut height = self.widget_data.geometry.requested_size.height;

        if self.widget_data.geometry.fill_width {
            width = available_size.width;
        }

        if self.widget_data.geometry.fill_height {
            height = available_size.height;
        }

        return Size{width,height}
    }
}

impl RenderableWidget for PanePar {
    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle_rec(self.widget_data.geometry.item_layout,Color::SKYBLUE)
    }
}


impl WidgetDataProvider for PanePar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}
