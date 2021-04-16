use crate::alignment::Alignment;
use raylib::core::math::Vector2;
use raylib::core::drawing::RaylibDrawHandle;
use crate::node_state::NodeState;
use crate::node_geometry::NodeGeometry;
use crate::node_model::NodeModel;
use crate::mouse::MouseState;
use crate::node::{NodePar, DirtyFlags};

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
}