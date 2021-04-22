use crate::widget_data::{WidgetData, SizeableWidget, WidgetDataProvider};
use crate::widget_operation::{DirtyFlags, RenderableWidget, WidgetOp};
use crate::gui::{GuiData, Gui};
use raylib::core::drawing::RaylibDrawHandle;
use vec_tree::{VecTree, Index};
use crate::widget::Widget;
use crate::size::{Size, CachedSize};
use raylib::math::Vector2;
use std::cell::Cell;

pub struct HBoxPar {
    widget_data: WidgetData,
    spacing: Cell<f32>,
}

impl HBoxPar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new(), spacing: Cell::new(10.0) }
    }

    pub fn set_spacing(&self, gui:&Gui, spacing: f32) -> &HBoxPar {
        if spacing.eq(&self.spacing.get()) {
            return self;
        }
        self.spacing.set(spacing);
        self.widget_data.invalidate_preferred_size(gui);
        self
    }

    pub fn get_spacing(&self) -> f32 {
        self.spacing.get()
    }
}

impl WidgetDataProvider for HBoxPar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}

impl SizeableWidget for HBoxPar {

    fn update_preferred_size(&self, gui: &Gui) {
        todo!()
    }


    fn update_content_size(&self, gui: &Gui, available_space: &Size) {
        todo!()
    }

}

impl RenderableWidget for HBoxPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, position:Vector2) {
        todo!();
    }
}