use crate::alignment::Alignment;
use raylib::core::math::Vector2;
use raylib::core::drawing::RaylibDrawHandle;
use crate::node_state::NodeState;
use crate::node_geometry::NodeGeometry;
use crate::node_model::NodeModel;
use crate::mouse::MouseState;
use crate::node::{NodePar, DirtyFlags, Size};

pub struct ButtonPar {
    state:NodeState,
    geometry:NodeGeometry,
    model:NodeModel,
}

impl NodePar for ButtonPar {

    fn content_width(&self) -> f32 {
        self.geometry.content_size.width
    }

    fn content_height(&self) -> f32 {
        self.geometry.content_size.height
    }

    fn padding(&self) -> f32 {
        self.model.padding
    }

    fn layout(&mut self) {

        todo!()
    }

    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState) {
        todo!()
    }

    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        todo!()
    }

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn NodePar {
        if self.geometry.target.eq(point) && self.geometry.alignment.eq(&alignment) {
            return self;
        }
        self.geometry.alignment = alignment;
        self.geometry.target.x = point.x;
        self.geometry.target.y = point.y;
        self.state.dirty_flags |= DirtyFlags::POSITION;
        self
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn NodePar {
        if padding == self.model.padding {
            return self;
        }
        self.model.padding = padding;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn clear_requested_size(&mut self) -> &mut dyn NodePar {
        self.geometry.requested_size = Size::none();
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn NodePar {
        if height == self.geometry.requested_size.height {
            return self;
        }
        self.geometry.requested_size.height = height;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn NodePar {
        if width == self.geometry.requested_size.width {
            return self;
        }
        self.geometry.requested_size.width = width;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn NodePar {
        self.set_requested_width(size.width);
        self.set_requested_height(size.height)
    }
}