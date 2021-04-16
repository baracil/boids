use crate::node_state::NodeState;
use crate::node_geometry::NodeGeometry;
use crate::node_model::NodeModel;
use crate::node::{NodePar, Size, DirtyFlags, FontInfo};
use crate::alignment::Alignment;
use raylib::prelude::*;
use crate::mouse::MouseState;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use raylib::math::Rectangle;
use raylib::color::Color;
use std::rc::Rc;

pub struct LabelPar {
    pub state: NodeState,
    pub geometry: NodeGeometry,
    pub model: NodeModel,
    pub text: Option<String>,
    pub font: FontInfo,
    pub spacing:f32, //todo use style to define this value
    pub color:Color,//todo use style to define this value
}


impl LabelPar {

    pub fn new(font_info:FontInfo) -> Self {
        Self {
            state:NodeState::new(),
            geometry:NodeGeometry::new(),
            model:NodeModel::new(),
            text:None,
            font:font_info,
            spacing:0.0,
            color:Color::BLACK
        }
    }


    // unset the dirty flag and return true if it was already unset
    fn unset_dirty_flag(&mut self, flag: DirtyFlags) -> bool {
        let mut dirty = self.state.dirty_flags;
        if (dirty & flag).is_empty() {
            return true;
        }
        dirty.remove(DirtyFlags::STYLE);
        return false;
    }

    fn compute_style(&mut self) {
        if self.unset_dirty_flag(DirtyFlags::STYLE) {
            return;
        }
    }

    fn compute_content_size(&mut self) {
        if self.unset_dirty_flag(DirtyFlags::CONTENT_SIZE) {
            return;
        }
        let text = match &self.text {
            None => "",
            Some(text) => text.as_str()
        };

        let text_size = self.font.measure_text(text, self.spacing);//TODO use style in self.state to get the spacing
        self.geometry.content_size = text_size;
    }

    fn compute_item_size(&mut self) {
        if self.unset_dirty_flag(DirtyFlags::SIZE) {
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
        if self.unset_dirty_flag(DirtyFlags::POSITION) {
            return;
        }
        self.geometry.copy_size_to_layout();
        self.geometry.compute_item_position();
        self.geometry.compute_content_position();
    }
}


impl NodePar for LabelPar {
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
        self.compute_style();
        self.compute_content_size();
        self.compute_item_size();
        self.compute_position();
    }

    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState) {

    }

    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(background) = &self.state.background {
            background.draw(d,&self.geometry.item_layout)
        }
        if let Some(border) = &self.state.border {
            border.draw(d,&self.geometry.item_layout)
        }

        d.draw_rectangle_rec(self.geometry.item_layout, Color::GREEN);

        if let Some(text) = &self.text {
            let position = Vector2 {x: self.geometry.content_layout.x, y: self.geometry.content_layout.y};
            self.font.draw_text(d,text.as_str(), &position, self.spacing, self.color)
        }
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