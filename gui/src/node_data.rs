use crate::node::{Parent, Node, Size, DirtyFlags, UpdatableNode, LayoutableNode};
use raylib::math::Vector2;
use crate::alignment::Alignment;
use crate::mouse::MouseState;
use raylib::drawing::RaylibDrawHandle;
use std::rc::Rc;
use crate::node_state::NodeState;
use crate::node_geometry::NodeGeometry;
use crate::node_model::NodeModel;


pub struct NodeData {
    pub parent: Option<Rc<dyn Node>>,
    pub children: Vec<Box<dyn Node>>,
    pub state: NodeState,
    pub geometry: NodeGeometry,
    pub model: NodeModel,
}

impl NodeData {
    pub fn new() -> Self {
        Self {
            parent:None,
            children:Vec::new(),
            state: NodeState::new(),
            geometry: NodeGeometry::new(),
            model: NodeModel::new(),
        }
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
        let mut width;
        let mut height;

        if self.geometry.requested_size.width > 0.0 {
            width = self.geometry.requested_size.width;
        } else {
            width = self.geometry.content_size.width + 2.0*self.model.padding;
        }

        if self.geometry.requested_size.height > 0.0 {
            height = self.geometry.requested_size.height;
        } else {
            height = self.geometry.content_size.height + 2.0*self.model.padding;
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

    fn layout(&mut self, node_base: &dyn NodeBase) {
        self.compute_style();
        let content_size = node_base.compute_content_size();
        self.geometry.content_size = content_size;
        self.compute_item_size();
        self.compute_position();
    }
}

impl<N: NodeBase> LayoutableNode for N {
    fn layout(&mut self) {
        self.node_data_mut().compute_style();
        let content_size = self.compute_content_size();
        self.node_data_mut().geometry.content_size = content_size;
        self.node_data_mut().compute_item_size();
        self.node_data_mut().compute_position();
    }
}


pub trait NodeBase {
    fn node_data(&self) -> &NodeData;
    fn node_data_mut(&mut self) -> &mut NodeData;
    fn compute_content_size(&self) -> Size;
}

impl Parent for NodeData {
    fn children(&self) -> &[Box<dyn Node>] {
        self.children.as_slice()
    }
}

impl Node for NodeData {
    fn parent(&self) -> &Option<Rc<dyn Node>> {
        &self.parent
    }

    fn content_width(&self) -> f32 {
        self.geometry.content_size.width
    }

    fn content_height(&self) -> f32 {
        self.geometry.content_size.height
    }

    fn padding(&self) -> f32 {
        self.model.padding
    }

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn Node {
        if self.geometry.target.eq(point) && self.geometry.alignment.eq(&alignment) {
            return self;
        }
        self.geometry.alignment = alignment;
        self.geometry.target.x = point.x;
        self.geometry.target.y = point.y;
        self.state.dirty_flags |= DirtyFlags::POSITION;
        self
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn Node {
        if padding == self.model.padding {
            return self;
        }
        self.model.padding = padding;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn clear_requested_size(&mut self) -> &mut dyn Node {
        self.geometry.requested_size = Size::empty();
        self
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn Node {
        if height == self.geometry.requested_size.height {
            return self;
        }
        self.geometry.requested_size.height = height;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn Node {
        if width == self.geometry.requested_size.width {
            return self;
        }
        self.geometry.requested_size.width = width;
        self.state.dirty_flags |= DirtyFlags::SIZE;
        self
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn Node {
        self.set_requested_width(size.width);
        self.set_requested_height(size.height)
    }
}


impl <N : NodeBase> Node for N {
    fn parent(&self) -> &Option<Rc<dyn Node>> {
        &self.node_data().parent
    }

    fn content_width(&self) -> f32 {
        self.node_data().content_width()
    }

    fn content_height(&self) -> f32 {
        self.node_data().content_height()
    }

    fn padding(&self) -> f32 {
        self.node_data().padding()
    }

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn Node {
        self.node_data_mut().set_position(point,alignment)
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn Node {
        self.node_data_mut().set_padding(padding)
    }

    fn clear_requested_size(&mut self) -> &mut dyn Node {
        self.node_data_mut().clear_requested_size()
    }

    fn set_requested_height(&mut self, height: f32) -> &mut dyn Node {
        self.node_data_mut().set_requested_height(height)
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn Node {
        self.node_data_mut().set_requested_width(width)
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn Node {
        self.node_data_mut().set_requested_size(size)
    }
}
