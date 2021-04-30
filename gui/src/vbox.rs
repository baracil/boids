use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetSpecific};
use crate::gui::{Gui};
use crate::size::{Size};
use std::cell::Cell;
use crate::fill::Fill;
use raylib::prelude::*;
use std::ops::Deref;

pub struct VBoxPar {
    widget_data: WidgetData,
    spacing: Cell<f32>,
}

impl Deref for VBoxPar {
    type Target = WidgetData;

    fn deref(&self) -> &Self::Target {
        &self.widget_data
    }
}

impl VBoxPar {
    pub fn new() -> Self {
        Self { widget_data: WidgetData::new(), spacing: Cell::new(10.0) }
    }

    pub fn set_spacing(&self, gui: &Gui, spacing: f32) -> &VBoxPar {
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

// impl WidgetDataProvider for VBoxPar {
//     fn widget_data(&self) -> &WidgetData {
//         &self.widget_data
//     }
//
//     fn widget_data_mut(&mut self) -> &mut WidgetData {
//         &mut self.widget_data
//     }
// }

impl WidgetSpecific for VBoxPar {

    fn get_widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn get_widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }

    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return Size::empty();
        }

        let tree_index = tree_index.unwrap();

        let mut nb_children = 0;
        let mut max_width: f32 = 0.0;
        let mut summed_height: f32 = 0.0;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let child_preferred_size = child.get_computed_size(gui);
                nb_children += 1;
                max_width = max_width.max(child_preferred_size.width());
                summed_height += child_preferred_size.height();
            }
        }

        let spacing = self.spacing.get();
        summed_height += spacing * ((nb_children - 1).max(0) as f32);

        let computed = Size::new(max_width, summed_height).with_padding(&self.get_padding());

        let mut preferred = self.get_preferred_size();
        preferred.replace_empty_dimensions_and_max(&computed);

        return preferred.clone();
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let mut summed_fixed_height:f32 = 0.0;
        let mut summed_weight:u32 = 0;
        let mut nb_children = 0;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let fill = child.fill_height();
                nb_children += 1;
                match fill {
                    Fill::Disabled => {
                        summed_fixed_height += child.get_computed_size(gui).height();
                    }
                    Fill::Enabled { weight} => {
                        summed_weight += weight;
                    }
                }
            }
        }

        let padding = self.get_padding();
        let width = available_size.width() - padding.h_padding();
        let height= available_size.height() - padding.v_padding();

        let space_taken_by_spacing = self.spacing.get() * ((nb_children - 1).max(0) as f32);


        let fill_height = (height-space_taken_by_spacing-summed_fixed_height)/(summed_weight.max(1) as f32);

        if width<0.0 || height<=0.0 {
            return
        }

        let mut size = Size::new(width,0.0);
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let fill = child.fill_height();
                match fill {
                    Fill::Disabled => {
                        let child_height = child.get_computed_size(gui).height();
                        size.set_height(child_height);
                        child.update_content_size(gui,&size);
                    }
                    Fill::Enabled { weight } => {
                        size.set_height(fill_height * weight as f32);
                        child.update_content_size(gui, &size)
                    }
                }
            }
        }

    }

    fn compute_child_positions(&self, gui: &Gui) {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let content_size = {
            let content_layout = self.get_content_layout();
            Size::new(content_layout.width, content_layout.height)
        };

        let spacing = self.spacing.get();

        let mut position = Vector2::new(0.0,0.0);
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                {
                    position.x = (content_size.width() - w.get_widget_width())*0.5;
                    w.set_widget_target(&position);
                    w.update_child_positions(gui);
                }
                position.y += w.get_widget_height() + spacing;
            }
        }
    }
}

impl RenderableWidget for VBoxPar {

    fn render(&self, gui: &Gui, d: &mut impl RaylibDraw, offset: &Vector2) {
        let tree_index = self.get_tree_index();
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        self.render_background_and_border(d, &offset);

        let content_layout = self.get_content_layout();
        let mut target = offset.clone();
        target.x += content_layout.x;
        target.y += content_layout.y;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                w.render(gui, d, &target);
            }
        }
    }
}