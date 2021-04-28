


use crate::label::LabelPar;
use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider};
use raylib::core::drawing::{RaylibDrawHandle, RaylibMode2D};
use crate::pane::PanePar;
use crate::gui::{Gui};
use crate::vbox::VBoxPar;
use crate::size::Size;
use raylib::math::Vector2;
use crate::hbox::HBoxPar;
use crate::slider::SliderPar;
use raylib::RaylibHandle;

pub enum  Widget {
    Label(LabelPar),
    Pane(PanePar),
    VBox(VBoxPar),
    HBox(HBoxPar),
    Slider(SliderPar)
}

impl LayoutableWidget for Widget {
    fn get_computed_size(&self, gui:&Gui) -> Size {
        match &self {
            Widget::Label(p) => p.get_computed_size(gui),
            Widget::Pane(p) => p.get_computed_size(gui),
            Widget::VBox(p) => p.get_computed_size(gui),
            Widget::HBox(p) => p.get_computed_size(gui),
            Widget::Slider(p) => p.get_computed_size(gui),
        }
    }

    fn update_content_size(&self, gui:&Gui, available_space:&Size) {
        match self {
            Widget::Label(p) => p.update_content_size(gui, available_space),
            Widget::Pane(p) => p.update_content_size(gui, available_space),
            Widget::VBox(p) => p.update_content_size(gui, available_space),
            Widget::HBox(p) => p.update_content_size(gui, available_space),
            Widget::Slider(p) => p.update_content_size(gui, available_space),
        }
    }

    fn update_child_positions(&self, gui: &Gui) {
        match self {
            Widget::Label(p) => p.update_child_positions(gui),
            Widget::Pane(p) => p.update_child_positions(gui),
            Widget::VBox(p) => p.update_child_positions(gui),
            Widget::HBox(p) => p.update_child_positions(gui),
            Widget::Slider(p) => p.update_child_positions(gui),
        }
    }
}

impl WidgetDataProvider for Widget {

    fn widget_data(&self) -> &WidgetData {
        match &self {
            Widget::Label(p) => p.widget_data(),
            Widget::Pane(p) => p.widget_data(),
            Widget::VBox(p) => p.widget_data(),
            Widget::HBox(p) => p.widget_data(),
            Widget::Slider(p) => p.widget_data(),
        }
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        match self {
            Widget::Label(p) => p.widget_data_mut(),
            Widget::Pane(p) => p.widget_data_mut(),
            Widget::VBox(p) => p.widget_data_mut(),
            Widget::HBox(p) => p.widget_data_mut(),
            Widget::Slider(p) => p.widget_data_mut(),
        }
    }
}

impl RenderableWidget for Widget {
    fn render(&self, gui:&Gui, d: &mut RaylibDrawHandle, offset:&Vector2) {
        match self {
            Widget::Label(p) => p.render(gui, d, offset),
            Widget::Pane(p) => p.render(gui, d, offset),
            Widget::VBox(p) => p.render(gui, d, offset),
            Widget::HBox(p) => p.render(gui, d, offset),
            Widget::Slider(p) => p.render(gui, d, offset),
        }
    }

}