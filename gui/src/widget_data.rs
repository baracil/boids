use raylib::math::Vector2;

use crate::alignment::Alignment;
use crate::mouse::MouseState;
use crate::widget_geometry::WidgetGeometry;
use crate::widget_model::WidgetModel;
use crate::widget_operation::{DirtyFlags, LayoutableWidget, Size, UpdatableWidget, WidgetOp};
use crate::widget_state::WidgetState;

pub struct WidgetData {
    pub state: WidgetState,
    pub geometry: WidgetGeometry,
    pub model: WidgetModel,
}

impl WidgetData {
    pub fn new() -> Self {
        Self {
            state: WidgetState::new(),
            geometry: WidgetGeometry::new(),
            model: WidgetModel::new(),
        }
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
        self.geometry.item_size.height = height
    }

    fn compute_position(&mut self) {
        if self.state.unset_dirty_flag(DirtyFlags::POSITION) {
            return;
        }
        self.geometry.copy_size_to_layout();
        self.geometry.compute_item_position();
        self.geometry.compute_content_position();
    }

    fn layout(&mut self, sizeable_node: &dyn SizeableWidget) {
        self.compute_style();
        let content_size = sizeable_node.compute_content_size();
        self.geometry.content_size = content_size;
        self.compute_item_size();
        self.compute_position();
    }
}

impl<N: WidgetDataProvider + SizeableWidget> LayoutableWidget for N {
    fn layout(&mut self) {
        self.widget_data_mut().compute_style();
        let content_size = self.compute_content_size();
        self.widget_data_mut().geometry.content_size = content_size;
        self.widget_data_mut().compute_item_size();
        self.widget_data_mut().compute_position();
    }
}

impl<N: WidgetDataProvider> UpdatableWidget for N {
    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState) {
        self.widget_data_mut().update(mouse_position,mouse_state)
    }
}

pub trait WidgetDataProvider {
    fn widget_data(&self) -> &WidgetData;
    fn widget_data_mut(&mut self) -> &mut WidgetData;
}

pub trait SizeableWidget {
    fn compute_content_size(&self) -> Size;
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

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn WidgetOp{
        if self.geometry.target.eq(point) && self.geometry.alignment.eq(&alignment) {
            return self;
        }
        self.geometry.alignment = alignment;
        self.geometry.target.x = point.x;
        self.geometry.target.y = point.y;
        self.state.dirty_flags |= DirtyFlags::POSITION;
        self
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn WidgetOp {
        if padding == self.model.padding {
            return self
        }
        self.model.padding = padding;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn clear_requested_size(&mut self) -> &mut dyn WidgetOp {
        self.geometry.requested_size = Size::empty();
        self
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn WidgetOp {
        if height == self.geometry.requested_size.height {
            return self
        }
        self.geometry.requested_size.height = height;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn WidgetOp {
        if width == self.geometry.requested_size.width {
            return self;
        }
        self.geometry.requested_size.width = width;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn WidgetOp {
        self.set_requested_width(size.width);
        self.set_requested_height(size.height)
    }
}

impl<M : WidgetDataProvider> WidgetOp for M {
    fn content_width(&self) -> f32 {
        self.widget_data().content_width()
    }

    fn content_height(&self) -> f32 {
        self.widget_data().content_height()
    }

    fn padding(&self) -> f32 {
        self.widget_data().padding()
    }

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn WidgetOp{
        self.widget_data_mut().set_position(point,alignment);
        self
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_padding(padding);
        self
    }

    fn clear_requested_size(&mut self) -> &mut dyn WidgetOp {
        self.widget_data_mut().clear_requested_size();
        self
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_height(height);
        self
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_width(width);
        self
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn WidgetOp {
        self.widget_data_mut().set_requested_size(size);
        self
    }
}