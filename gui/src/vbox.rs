use crate::widget_data::{WidgetData};
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider, SizeComputer, WidgetOp};
use crate::gui::{Gui};
use raylib::core::drawing::RaylibDrawHandle;
use crate::size::{Size};
use raylib::math::Vector2;
use std::cell::Cell;
use std::ops::Deref;
use crate::fill::Fill;
use crate::fill::Fill::Enabled;
use crate::alignment::HAlignment::Left;
use crate::alignment::VAlignment::Top;

pub struct VBoxPar {
    widget_data: WidgetData,
    spacing: Cell<f32>,
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

impl WidgetDataProvider for VBoxPar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}

impl SizeComputer for VBoxPar {
    fn compute_size(&self, gui: &Gui) -> Size {
        let tree_index = self.widget_data.tree_index;
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

        summed_height += self.spacing.get() * ((nb_children - 1).max(0) as f32);

        let computed = Size::new(max_width, summed_height).with_padding(&self.widget_data().model.padding.get());

        let mut preferred = self.widget_data.model.user_preferred_size.get();
        preferred.replace_empty_dimensions_and_max(&computed);

        return preferred.clone();
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let mut nb_children:u32 = 0;
        let mut nb_filled:u32 = 0;
        let mut nb_fixed:u32 = 0;
        let mut summed_fixed_height:f32 = 0.0;
        let mut summed_weight:u32 = 0;

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                nb_children+=1;
                let fill = child.widget_data().fill_height();
                match fill {
                    Fill::Disabled => {
                        summed_fixed_height += child.get_computed_size(gui).height();
                        nb_fixed += 1;
                    }
                    Fill::Enabled { weight} => {
                        summed_weight += weight;
                        nb_filled += 1;
                    }
                }
            }
        }

        let padding = self.widget_data.model.padding.get();
        let mut width = available_size.width() - padding.h_padding();
        let mut height= available_size.height() - padding.v_padding();

        let fix_height = summed_fixed_height/(nb_fixed.max(1) as f32);
        let fill_height = (height-summed_fixed_height)/(nb_filled.max(1) as f32);

        if width<0.0 || height<=0.0 {
            return
        }

        let mut size = Size::new(width,fix_height);
        for child_index in gui.get_widget_children(tree_index) {
            if let Some(child) = gui.get_widget(child_index) {
                let fill = child.widget_data().fill_height();
                match fill {
                    Fill::Disabled => {
                        size.set_height(fix_height);
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
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        let widget_layout = self.widget_data.geometry.widget_layout.borrow();
        let padding = self.widget_data.model.padding.get();
        let spacing = self.spacing.get();

        let mut position = Vector2::new(padding.left, padding.top);

        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let widget_size = w.widget_data().geometry.widget_size.borrow();

                w.widget_data().set_widget_target(&position);
                w.update_child_positions(gui);

                position.y += widget_layout.height + spacing;
            }
        }
    }
}

impl RenderableWidget for VBoxPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2, available_space:&Size) {
        let tree_index = self.widget_data.tree_index;
        if tree_index.is_none() {
            return;
        }
        let tree_index = tree_index.unwrap();

        self.widget_data.render_background_and_border(d, &offset);


        let widget_layout = self.widget_data.geometry.widget_layout.borrow();
        let mut inner_offset = offset.clone();
        inner_offset.x += widget_layout.x;
        inner_offset.y += widget_layout.y;


        for child_index in gui.get_widget_children(tree_index) {
            if let Some(w) = gui.get_widget(child_index) {
                let borrowed_size = w.widget_data().geometry.widget_size.borrow();
                let widget_size = borrowed_size.size();
                w.render(gui,d,&inner_offset, widget_size);
            }
        }

    }
}