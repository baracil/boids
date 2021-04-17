use crate::node_state::NodeState;
use crate::node_geometry::NodeGeometry;
use crate::node_model::NodeModel;
use crate::node::{Node, Size, DirtyFlags, UpdatableNode, LayoutableNode, RenderableNode};
use crate::alignment::Alignment;
use raylib::prelude::*;
use crate::mouse::MouseState;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use raylib::math::Rectangle;
use raylib::color::Color;
use std::rc::Rc;
use crate::font::FontInfo;
use crate::node_data::{NodeData, NodeBase, SizeableNode};

pub struct Label {
    pub data: NodeData,
    text: Option<String>,
    font: FontInfo,
    pub spacing:f32, //todo use style to define this value
    pub color:Color,//todo use style to define this value
}


impl Label {

    pub fn new(font_info:FontInfo) -> Self {
        Self {
            data:NodeData::new(),
            text:None,
            font:font_info,
            spacing:0.0,
            color:Color::BLACK
        }
    }

    pub fn clear_text(&mut self) -> &mut Label {
        if let Some(_) = self.text {
            self.data.set_dirty_flag(DirtyFlags::SIZE);
            self.text = None;
        }
        self
    }

    pub fn set_text(&mut self, text:String) -> &mut Label {
        if let Some(txt) = &self.text {
            if text.eq(txt) {
                return self;
            }
        }
        self.data.set_dirty_flag(DirtyFlags::SIZE);
        self.text = Some(text);
        self
    }

}

impl NodeBase for Label {
    fn node_data(&self) -> &NodeData {
        return &self.data;
    }

    fn node_data_mut(&mut self) -> &mut NodeData {
        return &mut self.data;
    }
}

impl SizeableNode for Label {
    fn compute_content_size(&self) -> Size {
        let text = match &self.text {
            None => "",
            Some(text) => text.as_str()
        };

        self.font.measure_text(text, self.spacing) //TODO use style in self.state to get the spacing
    }
}

impl RenderableNode for Label {

    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(background) = &self.data.state.background {
            background.draw(d,&self.data.geometry.item_layout)
        }
        if let Some(border) = &self.data.state.border {
            border.draw(d,&self.data.geometry.item_layout)
        }

        d.draw_rectangle_rec(self.data.geometry.item_layout, Color::GREEN);

        if let Some(text) = &self.text {
            let position = Vector2 {x: self.data.geometry.content_layout.x, y: self.data.geometry.content_layout.y};
            self.font.draw_text(d,text.as_str(), &position, self.spacing, self.color)
        }
    }

}