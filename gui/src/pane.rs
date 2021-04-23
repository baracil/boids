
use raylib::prelude::*;

use crate::gui::{Gui};
use crate::widget_data::{SizeableWidget, WidgetData, WidgetDataProvider};
use crate::widget_operation::{RenderableWidget};
use crate::size::{Size};

pub struct PanePar {
    widget_data: WidgetData,

}

impl PanePar {
    pub fn new() -> Self {
        Self{widget_data:WidgetData::new()}
    }
}

impl SizeableWidget for PanePar {

    fn update_preferred_size(&self, gui: &Gui) {
        todo!()
    }

    fn update_content_size(&self, gui: &Gui, available_space: &Size) {
        todo!()
    }
}

impl RenderableWidget for PanePar {
    fn render(&self, gui:&Gui, d: &mut RaylibDrawHandle<'_>, position:Vector2) {
        let layout = self.widget_data.geometry.widget_layout.to_owned().into_inner();
        d.draw_rectangle_rec(layout, Color::SKYBLUE)
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
