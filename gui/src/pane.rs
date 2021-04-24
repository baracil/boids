use raylib::prelude::*;

use crate::gui::{Gui};
use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider, SizeComputer, WidgetOp};
use crate::size::{Size};
use crate::fill::Fill::Enabled;

pub struct PanePar {
    widget_data: WidgetData,

}

impl PanePar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new() }
    }
}

impl SizeComputer for PanePar {
    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return Size::empty();
        }
        let tree_index = tree_index.unwrap();

        let mut xmin:f32 = 0.0;
        let mut xmax:f32 = 0.0;
        let mut ymin:f32 = 0.0;
        let mut ymax:f32 = 0.0;
        let mut max_size = Size::empty();
        let mut first = true;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let preferred = w.get_computed_size(gui);
                let target = w.widget_data().model.position.get();
                if w.widget_data().is_fill_width_or_relative_x() {
                    if first {
                        max_size.set_height(preferred.height());
                        max_size.set_width(preferred.width());
                    } else {
                        max_size.max_mut(&preferred);
                    }
                } else {
                    if first {
                        xmin = target.x;
                        xmax = target.x + preferred.width();
                        ymin = target.y;
                        ymax = target.y + preferred.height();
                    } else {
                        xmin = xmin.min(target.x);
                        xmax = xmax.max(target.x + preferred.width());
                        ymin = ymin.min(target.y);
                        ymax = ymax.max(target.y + preferred.height());
                    }
                }
                first = false
            }
        }

        let pref_width = (xmax-xmin).max(max_size.width());
        let pref_height = (ymax-ymin).max(max_size.height());

        let children_size = Size::new(pref_width,pref_height).with_padding(&self.widget_data.model.padding.get());



        let mut user_preferred_size = self.widget_data.model.preferred_size.get();

        user_preferred_size.replace_empty_dimensions_and_max(&children_size);
        user_preferred_size
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let available_size_for_children = available_size.without_padding(&self.widget_data.model.padding.get());

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.update_content_size(gui, &available_size_for_children);
            }
        }
    }

    fn compute_child_positions(&self, gui: &Gui) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let content_size = {
            let content_layout = self.widget_data().geometry.content_layout.borrow();
            Size::new(content_layout.width,content_layout.height)
        };

        let mut target = Vector2::default();

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let child_widget_data = w.widget_data();

                child_widget_data.compute_default_target(&content_size);
                w.update_child_positions(gui)
            }
        }

    }
}

impl RenderableWidget for PanePar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2, available_space:&Size) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let mut target = offset.clone();
        let widget_layout = self.widget_data.geometry.widget_layout.borrow();
        target.x+= widget_layout.x;
        target.y+= widget_layout.y;


        self.widget_data.render_background_and_border(d,&offset);

        let widget_size = self.widget_data.geometry.widget_size.borrow().size().clone();
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.render(gui,d,&target,&widget_size)
            }
        }

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
