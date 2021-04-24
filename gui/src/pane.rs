use raylib::prelude::*;

use crate::gui::{Gui};
use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider, WidgetSpecific};
use crate::size::{Size};
use crate::position::Coordinate::{Absolute};
use crate::mouse::MouseState;

pub struct PanePar {
    widget_data: WidgetData,

}

impl PanePar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new() }
    }
}

impl WidgetSpecific for PanePar {
    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return Size::empty();
        }
        let tree_index = tree_index.unwrap();

        let mut xmin: f32 = 0.0;
        let mut xmax: f32 = 0.0;
        let mut ymin: f32 = 0.0;
        let mut ymax: f32 = 0.0;
        let mut max_size = Size::empty();
        let mut first_x = true;
        let mut first_y = true;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let preferred = w.get_computed_size(gui);
                let target = w.widget_data().model.position.get();

                match (target.get_x(), w.widget_data().is_fill_width(), first_x) {
                    (Absolute(value), false, true) => {
                        xmin = *value;
                        xmax = *value + preferred.width();
                        first_x = false
                    }
                    (Absolute(value), false, false) => {
                        xmin = xmin.min(*value);
                        xmax = xmax.max(*value + preferred.width());
                    }
                    (_, _, _) => {
                        max_size.max_width_mut(&preferred);
                    }
                }

                match (target.get_y(), w.widget_data().is_fill_height(), first_y) {
                    (Absolute(value), false, true) => {
                        ymin = *value;
                        ymax = *value + preferred.height();
                        first_y = false
                    }
                    (Absolute(value), false, false) => {
                        ymin = ymin.min(*value);
                        ymax = ymax.max(*value + preferred.height());
                    }
                    (_, _, _) => {
                        max_size.max_height_mut(&preferred);
                    }
                }
            }
        }

        let pref_width = (xmax - xmin).max(max_size.width());
        let pref_height = (ymax - ymin).max(max_size.height());

        let children_size = Size::new(pref_width, pref_height).with_padding(&self.widget_data.model.padding.get());

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
            let content_layout = self.widget_data().geometry.content_layout.get();
            Size::new(content_layout.width, content_layout.height)
        };

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
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let padding = self.widget_data.model.padding.get();
        let mut target = offset.clone();
        let widget_layout = self.widget_data.geometry.widget_layout.get();
        target.x += widget_layout.x;
        target.y += widget_layout.y;


        self.widget_data.render_background_and_border(d, &offset);

        target.x += padding.left;
        target.y += padding.top;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.render(gui, d, &target)
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
