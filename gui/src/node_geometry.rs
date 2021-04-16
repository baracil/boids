use raylib::math::{Rectangle, Vector2};
use crate::alignment::Alignment;
use crate::node::Size;

pub struct NodeGeometry {
    pub target: Vector2,           //the requested position where the node should be drawn
    pub alignment: Alignment,      //alignment to the target
    pub requested_size: Size,   //requested size of the node

    pub content_size: Size,     //size of the content
    pub content_layout: Rectangle, //layout of the content
    pub back_layout: Rectangle,    //layout of the background
}

impl NodeGeometry {
    pub(crate) fn new() -> Self {
        Self {
            target: Vector2::new(0.0, 0.0),
            alignment: Alignment::new(),
            back_layout: Default::default(),
            content_layout: Default::default(),
            content_size: Size::new(),
            requested_size: Size::new(),
        }
    }
}
