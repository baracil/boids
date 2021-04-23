use std::cell::{Cell};
use std::ops::BitAnd;

use generational_arena::Index;
use raylib::prelude::*;
use vec_tree::VecTree;

use crate::alignment::{HAlignment, VAlignment};
use crate::fill::Fill;
use crate::fill::Fill::Disabled;
use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::padding::Padding;
use crate::size::Size;
use crate::widget::Widget;
use crate::widget_geometry::WidgetGeometry;
use crate::widget_model::WidgetModel;
use crate::widget_operation::{DirtyFlags, UpdatableWidget, WidgetOp};
use crate::widget_state::WidgetState;

pub struct WidgetData {
    pub tree_index: Option<Index>,
    pub state: WidgetState,
    pub geometry: WidgetGeometry,
    pub model: WidgetModel,
}

impl WidgetData {

    fn get_parent<'a>(&self, gui: &'a Gui) -> Option<&'a Widget> {
        match self.tree_index {
            None => None,
            Some(idx) => {
                gui.get_parent_widget(idx)
            }
        }
    }

    pub fn invalidate_preferred_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::PREFERRED_SIZE);
    }

    pub fn invalidate_content_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::CONTENT_SIZE);
    }

    pub fn invalidate_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::SIZE);
    }

    pub fn invalidate_position(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::POSITION);
    }

    fn invalidate_flag(&self, gui:&Gui, flag:DirtyFlags) {
        if self.is_dirty_flag_set(flag) {
            return
        }
        self.set_dirty_flag(flag);
        if let Some(parent) = self.get_parent(gui) {
            parent.widget_data().invalidate_flag(gui,flag)
        }
    }
}

impl WidgetData {
    pub fn new() -> Self {
        Self {
            tree_index: None,
            state: WidgetState::new(),
            geometry: WidgetGeometry::new(),
            model: WidgetModel::new(),
        }
    }

    pub fn set_alignment(&self, gui: &Gui, valignment: VAlignment, haligment: HAlignment) {
        let mut current_alignment = self.geometry.alignment.get();
        if current_alignment.vertical.eq(&valignment) && current_alignment.horizontal.eq(&haligment) {
            return;
        }
        current_alignment.vertical = valignment;
        current_alignment.horizontal = haligment;
        self.geometry.alignment.set(current_alignment);
        self.invalidate_position(gui)
    }

    pub fn set_tree_index(&mut self, tree_index: Index) {
        self.tree_index = Some(tree_index);
    }

    pub fn clear_tree_index(&mut self) {
        self.tree_index = None
    }

    pub fn fill_width(&self) -> Fill {
        self.geometry.fill_width.get()
    }

    pub fn fill_height(&self) -> Fill {
        self.geometry.fill_height.get()
    }

    pub fn update(&mut self, _mouse_position: &Vector2, _mouse_state: &MouseState) {
        todo!()
    }

    pub fn set_dirty_flag(&self, flag: DirtyFlags) {
        self.state.dirty_flags.set(self.state.dirty_flags.get() | flag);
    }

    pub fn is_dirty_flag_set(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flags.get().bitand(flag).eq(&flag)
    }

    pub fn dirty_flag_clean(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flag_clean(flag)
    }

    pub fn update_widget_size(&self, gui: &Gui) {
        if self.state.dirty_flag_clean(DirtyFlags::SIZE) {
            return;
        }
        let borrowed_content_size = self.geometry.content_size.borrow();
        let content_size = borrowed_content_size.size();

        self.geometry.widget_size.replace(content_size.with_padding(&self.model.padding.get()));
        self.invalidate_position(gui);
    }

    pub fn compute_position(&mut self, available_size: &Size) {
        if self.state.dirty_flag_clean(DirtyFlags::POSITION) {
            return;
        }
        self.geometry.copy_size_to_layout();
        self.geometry.compute_item_position(available_size);
        self.geometry.compute_content_position();
    }

    fn disable_fill(&self, gui: &Gui, fill: &Cell<Fill>) {
        if fill.get().is_disabled() {
            return;
        }
        fill.set(Disabled);
        self.invalidate_preferred_size(gui)
    }

    fn enable_fill(&self, gui: &Gui, fill_cell: &Cell<Fill>, fill: Fill) {
        let current_fill = fill_cell.get();
        if current_fill.eq(&fill) {
            return;
        }

        fill_cell.set(fill);
        self.invalidate_preferred_size(gui)
    }

    fn set_absolute(&self, gui: &Gui, absolute_cell: &Cell<bool>, absolute: bool) {
        let current = absolute_cell.get();
        if current == absolute {
            return;
        }
        absolute_cell.set(absolute);
        self.invalidate_preferred_size(gui)
    }

    pub fn get_myself<'a>(&self, tree: &'a VecTree<Widget>) -> Option<&'a Widget> {
        self.tree_index.and_then(|idx| {
            tree.get(idx)
        })
    }
}

// impl<N: WidgetDataProvider + SizeableWidget> LayoutableWidget for N {
//     fn layout(&mut self, available_size: &Size) {
// {
//     self.widget_data().gui_data
// }
//         {
// //            compute_tree_style(&self.myself());
//         }
//
//         {
//             let children = self.children();
//             let mut sum_width_weight: u32 = 0;
//             let mut sum_height_weight: u32 = 0;
//             let mut used_width = 0.0;
//             let mut used_height = 0.0;
//             for child in children {
//                 // let data = child.borrow().widget_data();
//                 // sum_height_weight = data.fill_height_weight();
//                 // sum_width_weight = data.fill_width_weight();
//             }
//         }
//
//         if !self.widget_data_mut().unset_dirty_flag(DirtyFlags::CONTENT_SIZE) {
//             let content_size = self.compute_content_size(available_size);
//             self.widget_data_mut().geometry.content_size = content_size;
//         }
//
//         self.widget_data_mut().compute_item_size();
//         self.widget_data_mut().compute_position();
//     }
// }


impl<N: WidgetDataProvider> UpdatableWidget for N {
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState) {
        self.widget_data_mut().update(mouse_position, mouse_state)
    }
}

pub trait WidgetDataProvider {
    fn widget_data(&self) -> &WidgetData;
    fn widget_data_mut(&mut self) -> &mut WidgetData;
}

pub trait SizeableWidget {
    fn update_preferred_size(&self, gui: &Gui);
    fn update_content_size(&self, gui: &Gui, available_space: &Size);
}


impl WidgetOp for WidgetData {
    fn content_width(&self) -> f32 {
        self.geometry.content_size.borrow().size().width()
    }
    fn content_height(&self) -> f32 {
        self.geometry.content_size.borrow().size().height()
    }

    fn padding(&self) -> Padding {
        self.model.padding.get()
    }

    fn set_absolute_coordinate_y(&self, gui: &Gui, absolute: bool) -> &dyn WidgetOp {
        self.set_absolute(gui, &self.geometry.absolute_coordinate_y, absolute);
        self
    }

    fn set_absolute_coordinate_x(&self, gui: &Gui, absolute: bool) -> &dyn WidgetOp {
        self.set_absolute(gui, &self.geometry.absolute_coordinate_x, absolute);
        self
    }


    fn set_position(&self, gui: &Gui, x: f32, y: f32) -> &dyn WidgetOp {
        let mut current_position = self.geometry.target.get();
        if current_position.x.eq(&x) && current_position.y.eq(&y) {
            return self;
        }
        current_position.x = x;
        current_position.y = y;
        self.geometry.target.set(current_position);
        //dirty
        self
    }

    fn set_valignment(&self, gui: &Gui, valignment: VAlignment) -> &dyn WidgetOp {
        let current_alignment = self.geometry.alignment.get();
        self.set_alignment(gui, valignment, current_alignment.horizontal);
        self
    }

    fn set_halignment(&self, gui: &Gui, halignment: HAlignment) -> &dyn WidgetOp {
        let current_alignment = self.geometry.alignment.get();
        self.set_alignment(gui, current_alignment.vertical, halignment);
        self
    }

    fn set_padding(&self, gui: &Gui, padding: Padding) -> &dyn WidgetOp {
        let current_padding = self.model.padding.get();
        if current_padding.eq(&padding) {
            return self;
        }
        self.model.padding.set(padding);
        self.invalidate_preferred_size(gui);
        self
    }

    fn clear_requested_size(&self, gui: &Gui) -> &dyn WidgetOp {
        self.set_requested_size(gui, Size::empty());
        self.invalidate_preferred_size(gui);
        self
    }

    fn set_requested_height(&self, gui: &Gui, height: f32) -> &dyn WidgetOp {
        let size = self.geometry.requested_size.get().with_height(height);
        self.set_requested_size(gui, size);
        self.invalidate_preferred_size(gui);
        self
    }

    fn set_requested_width(&self, gui: &Gui, width: f32) -> &dyn WidgetOp {
        let size = self.geometry.requested_size.get().with_width(width);
        self.set_requested_size(gui, size);
        self.invalidate_preferred_size(gui);
        self
    }

    fn set_requested_size(&self, gui: &Gui, size: Size) -> &dyn WidgetOp {
        let current = self.geometry.requested_size.get();
        if current.eq(&size) {
            return self;
        }
        self.geometry.requested_size.set(size);
        self.invalidate_preferred_size(gui);
        self
    }

    fn disable_fill_width(&self, gui: &Gui) -> &dyn WidgetOp {
        self.disable_fill(gui, &self.geometry.fill_width);
        self
    }

    fn disable_fill_height(&self, gui: &Gui) -> &dyn WidgetOp {
        self.disable_fill(gui, &self.geometry.fill_height);
        self
    }

    fn enable_fill_width(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.enable_fill(gui, &self.geometry.fill_width, fill);
        self
    }

    fn enable_fill_height(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.enable_fill(gui, &self.geometry.fill_height, fill);
        self
    }
}

impl<M: WidgetDataProvider> WidgetOp for M {
    fn content_width(&self) -> f32 {
        self.widget_data().content_width()
    }

    fn content_height(&self) -> f32 {
        self.widget_data().content_height()
    }

    fn padding(&self) -> Padding {
        self.widget_data().padding()
    }

    fn set_absolute_coordinate_x(&self, gui: &Gui, absolute: bool) -> &dyn WidgetOp {
        self.widget_data().set_absolute_coordinate_x(gui, absolute)
    }

    fn set_absolute_coordinate_y(&self, gui: &Gui, absolute: bool) -> &dyn WidgetOp {
        self.widget_data().set_absolute_coordinate_y(gui, absolute)
    }

    fn set_position(&self, gui: &Gui, x: f32, y: f32) -> &dyn WidgetOp {
        self.widget_data().set_position(gui, x, y)
    }

    fn set_valignment(&self, gui: &Gui, valignment: VAlignment) -> &dyn WidgetOp {
        self.widget_data().set_valignment(gui, valignment)
    }

    fn set_halignment(&self, gui: &Gui, halignment: HAlignment) -> &dyn WidgetOp {
        self.widget_data().set_halignment(gui, halignment)
    }

    fn set_padding(&self, gui: &Gui, padding: Padding) -> &dyn WidgetOp {
        self.widget_data().set_padding(gui, padding)
    }

    fn clear_requested_size(&self, gui: &Gui) -> &dyn WidgetOp {
        self.widget_data().clear_requested_size(gui)
    }

    fn set_requested_height(&self, gui: &Gui, height: f32) -> &dyn WidgetOp {
        self.widget_data().set_requested_height(gui, height)
    }

    fn set_requested_width(&self, gui: &Gui, width: f32) -> &dyn WidgetOp {
        self.widget_data().set_requested_width(gui, width)
    }

    fn set_requested_size(&self, gui: &Gui, size: Size) -> &dyn WidgetOp {
        self.widget_data().set_requested_size(gui, size)
    }

    fn disable_fill_width(&self, gui: &Gui) -> &dyn WidgetOp {
        self.widget_data().disable_fill_width(gui)
    }

    fn disable_fill_height(&self, gui: &Gui) -> &dyn WidgetOp {
        self.widget_data().disable_fill_height(gui)
    }

    fn enable_fill_width(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.widget_data().enable_fill_width(gui, fill)
    }

    fn enable_fill_height(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.widget_data().enable_fill_height(gui, fill)
    }
}