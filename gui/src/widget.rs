use std::cell::RefCell;
use std::rc::Rc;



use crate::label::LabelPar;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};
use crate::widget_operation::{RenderableWidget};
use raylib::core::drawing::RaylibDrawHandle;
use crate::pane::PanePar;
use crate::gui::{Gui, GuiData};
use vec_tree::{Index, VecTree};
use crate::hbox::HBoxPar;
use crate::size::Size;
use raylib::math::Vector2;

pub enum  Widget {
    Label(LabelPar),
    Pane(PanePar),
    HBox(HBoxPar)
}

impl SizeableWidget for Widget {
    fn update_preferred_size(&self, gui:&Gui) {
        match &self {
            Widget::Label(p) => p.update_preferred_size(gui),
            Widget::Pane(p) => p.update_preferred_size(gui),
            Widget::HBox(p) => p.update_preferred_size(gui),
        }
    }

    fn update_content_size(&self, gui:&Gui, available_space:&Size) {
        match self {
            Widget::Label(p) => p.update_content_size(gui, available_space),
            Widget::Pane(p) => p.update_content_size(gui, available_space),
            Widget::HBox(p) => p.update_content_size(gui, available_space),
        }
    }

}

impl WidgetDataProvider for Widget {

    fn widget_data(&self) -> &WidgetData {
        match &self {
            Widget::Label(p) => p.widget_data(),
            Widget::Pane(p) => p.widget_data(),
            Widget::HBox(p) => p.widget_data(),
        }
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        match self {
            Widget::Label(p) => p.widget_data_mut(),
            Widget::Pane(p) => p.widget_data_mut(),
            Widget::HBox(p) => p.widget_data_mut(),
        }
    }
}

impl RenderableWidget for Widget {
    fn render(&self, gui:&Gui, d: &mut RaylibDrawHandle, position:Vector2) {
        match self {
            Widget::Label(p) => p.render(gui, d, position),
            Widget::Pane(p) => p.render(gui, d, position),
            Widget::HBox(p) => p.render(gui, d, position),
        }
    }
}