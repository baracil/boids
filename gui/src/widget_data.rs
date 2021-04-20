use raylib::math::Vector2;

use crate::alignment::{Alignment, HAlignment, VAlignment};
use crate::mouse::MouseState;
use crate::widget_geometry::WidgetGeometry;
use crate::widget_model::WidgetModel;
use crate::widget_operation::{DirtyFlags, LayoutableWidget, Size, UpdatableWidget, WidgetOp};
use crate::widget_state::WidgetState;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use uuid::Uuid;
use raylib::prelude::Color;
use raylib::drawing::RaylibDrawHandle;
use crate::widget::Widget;
use generational_arena::Index;
use std::mem::take;
use crate::gui::{Gui, GuiData, RefGuiData};

pub struct WidgetData {
    pub gui_data: Option<RefGuiData>,
    pub state: WidgetState,
    pub geometry: WidgetGeometry,
    pub model: WidgetModel,
}

impl WidgetData {

    pub fn new(gui_data:RefGuiData) -> Self {
        Self {
            gui_data,
            state: WidgetState::new(),
            geometry: WidgetGeometry::new(),
            model: WidgetModel::new(),
        }
    }

    pub fn is_fill_width(&self) -> bool {
        self.geometry.fill_width
    }

    pub fn is_fill_height(&self) -> bool {
        self.geometry.fill_height
    }

    pub fn fill_width_weight(&self) -> u32 {
        if self.geometry.fill_width {
            return self.geometry.fill_width_weight;
        }
        return 0;
    }

    pub fn fill_height_weight(&self) -> u32 {
        if self.geometry.fill_height {
            return self.geometry.fill_height_weight;
        }
        return 0;
    }

    pub fn update(&mut self, _mouse_position: &Vector2, _mouse_state: &MouseState) {
        todo!()
    }

    pub fn set_dirty_flag(&mut self, flag: DirtyFlags) {
        self.state.dirty_flags |= flag;
    }

    pub fn unset_dirty_flag(&mut self, flag: DirtyFlags) -> bool {
        self.state.unset_dirty_flag(flag)
    }

    fn compute_style(&mut self) {
        if self.state.unset_dirty_flag(DirtyFlags::STYLE) {
            return;
        }
        self.set_dirty_flag(DirtyFlags::CONTENT_SIZE)
    }

    fn compute_item_size(&mut self) {
        if self.state.unset_dirty_flag(DirtyFlags::SIZE) {
            return;
        }
        let width;
        let height;

        if self.geometry.requested_size.width > 0.0 {
            width = self.geometry.requested_size.width;
        } else {
            width = self.geometry.content_size.width + 2.0 * self.model.padding;
        }

        if self.geometry.requested_size.height > 0.0 {
            height = self.geometry.requested_size.height;
        } else {
            height = self.geometry.content_size.height + 2.0 * self.model.padding;
        }
        self.geometry.item_size.width = width;
        self.geometry.item_size.height = height;
        self.set_dirty_flag(DirtyFlags::POSITION)
    }

    fn compute_position(&mut self) {
        if self.state.unset_dirty_flag(DirtyFlags::POSITION) {
            return;
        }
        self.geometry.copy_size_to_layout();
        self.geometry.compute_item_position();
        self.geometry.compute_content_position();
    }
}

impl<N: WidgetDataProvider + SizeableWidget> LayoutableWidget for N {
    fn layout(&mut self, available_size: &Size) {
        {
            self.widget_data().gui_data
        }
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
    }
}

// pub fn compute_tree_style(node: &RefNode<Widget>) {
//     let mut bn = node.borrow_mut();
//
//     bn.widget_data_mut()
//         .compute_style();
//     for child in node.children() {
//         compute_tree_style(&child)
//     }
// }
//

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
    fn compute_content_size(&self, available_size: &Size) -> Size;
}

impl WidgetOp for WidgetData {
    fn content_width(&self) -> f32 {
        self.geometry.content_size.width
    }

    fn content_height(&self) -> f32 {
        self.geometry.content_size.height
    }

    fn padding(&self) -> f32 {
        self.model.padding
    }

    fn set_position(&mut self, x: f32, y: f32) -> &mut dyn WidgetOp {
        if self.geometry.target.x.eq(&x) && self.geometry.target.y.eq(&y) {
            return self;
        }
        self.geometry.target.x = x;
        self.geometry.target.y = y;
        self
    }

    fn set_valignment(&mut self, valignment: VAlignment) -> &mut dyn WidgetOp {
        if self.geometry.alignment.vertical.eq(&valignment) {
            return self;
        }
        self.geometry.alignment.vertical = valignment;
        self.set_dirty_flag(DirtyFlags::POSITION);
        self
    }

    fn set_halignment(&mut self, halignment: HAlignment) -> &mut dyn WidgetOp {
        if self.geometry.alignment.horizontal.eq(&halignment) {
            return self;
        }
        self.geometry.alignment.horizontal = halignment;
        self.set_dirty_flag(DirtyFlags::POSITION);
        self
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn WidgetOp {
        if padding == self.model.padding {
            return self;
        }
        self.model.padding = padding;
        self.set_dirty_flag(DirtyFlags::SIZE);
        self
    }

    fn clear_requested_size(&mut self) -> &mut dyn WidgetOp {
        self.geometry.requested_size = Size::empty();
        self.set_dirty_flag(DirtyFlags::SIZE);
        self
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn WidgetOp {
        if height == self.geometry.requested_size.height {
            return self;
        }
        self.geometry.requested_size.height = height;
        self.set_dirty_flag(DirtyFlags::SIZE);
        self
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn WidgetOp {
        if width == self.geometry.requested_size.width {
            return self;
        }
        self.geometry.requested_size.width = width;
        self.set_dirty_flag(DirtyFlags::SIZE);
        self
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn WidgetOp {
        self.set_requested_width(size.width);
        self.set_requested_height(size.height)
    }

    fn set_fill_width(&mut self, fill: bool) -> &mut dyn WidgetOp {
        if fill == self.geometry.fill_width {
            return self;
        }
        self.geometry.fill_width = fill;
        self.set_dirty_flag(DirtyFlags::CONTENT_SIZE);
        self
    }

    fn set_fill_height(&mut self, fill: bool) -> &mut dyn WidgetOp {
        if fill == self.geometry.fill_height {
            return self;
        }
        self.geometry.fill_height = fill;
        self.set_dirty_flag(DirtyFlags::CONTENT_SIZE);
        self
    }

    fn set_fill_width_weight(&mut self, weight: u32) -> &mut dyn WidgetOp {
        if weight == self.geometry.fill_width_weight {
            return self;
        }
        self.geometry.fill_width_weight = weight;
        self.set_dirty_flag(DirtyFlags::CONTENT_SIZE);
        self
    }

    fn set_fill_height_weight(&mut self, weight: u32) -> &mut dyn WidgetOp {
        if weight == self.geometry.fill_height_weight {
            return self;
        }
        self.geometry.fill_height_weight = weight;
        self.set_dirty_flag(DirtyFlags::CONTENT_SIZE);
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

    fn padding(&self) -> f32 {
        self.widget_data().padding()
    }

    fn set_position(&mut self, x: f32, y: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_position(x,y)
    }

    fn set_valignment(&mut self, valignment: VAlignment) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_valignment(valignment)
    }

    fn set_halignment(&mut self, halignment: HAlignment) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_halignment(halignment)
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_padding(padding)
    }

    fn clear_requested_size(&mut self) -> &mut dyn WidgetOp {
        self.widget_data_mut().clear_requested_size()
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_height(height)
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_width(width)
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_size(size)
    }

    fn set_fill_width(&mut self, fill: bool) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_fill_width(fill)
    }

    fn set_fill_height(&mut self, fill: bool) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_fill_height(fill)
    }

    fn set_fill_width_weight(&mut self, weight: u32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_fill_width_weight(weight)
    }

    fn set_fill_height_weight(&mut self, weight: u32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_fill_height_weight(weight)
    }
}