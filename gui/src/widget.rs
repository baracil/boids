use std::cell::RefCell;
use std::rc::Rc;


use tree::tree::{TreeData, TreeDataProvider};

use crate::label::LabelPar;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};
use crate::widget_operation::{Size, RenderableWidget};
use raylib::core::drawing::RaylibDrawHandle;

pub enum  Widget {
    Label(LabelPar)
}

impl TreeDataProvider<Widget> for Widget {
    fn tree_data(&self) -> Rc<RefCell<TreeData<Widget>>> {
        match &self {
            Widget::Label(p) => p.tree_data()
        }
    }
}

impl SizeableWidget for Widget {
    fn compute_content_size(&self) -> Size {
        match &self {
            Widget::Label(p) => p.compute_content_size(),
        }
    }
}

impl WidgetDataProvider for Widget {

    fn widget_data(&self) -> &WidgetData {
        match &self {
            Widget::Label(p) => p.widget_data(),
        }
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        match self {
            Widget::Label(p) => p.widget_data_mut(),
        }
    }
}

impl RenderableWidget for Widget {
    fn render(&self, d: &mut RaylibDrawHandle) {
        match self {
            Widget::Label(p) => p.render(d)
        }
    }
}