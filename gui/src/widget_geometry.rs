use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use crate::widget_operation::Size;

pub struct WidgetGeometry {
    pub target: Vector2,      //the requested position where the node should be drawn
    pub alignment: Alignment, //alignment to the target
    pub requested_size: Size, //requested size of the node

    pub content_size: Size, //size of the content
    pub item_size: Size,    // size of the item

    pub content_layout: Rectangle, //layout of the content
    pub item_layout: Rectangle,    //layout of the background
}

impl WidgetGeometry {
    pub(crate) fn new() -> Self {
        Self {
            target: Vector2::new(0.0, 0.0),
            alignment: Alignment::new(),
            item_layout: Default::default(),
            content_layout: Default::default(),
            content_size: Size::empty(),
            item_size: Size::empty(),
            requested_size: Size {
                width: -1.0,
                height: -1.0,
            },
        }
    }

    pub fn copy_size_to_layout(&mut self) {
        self.content_layout.width = self.content_size.width;
        self.content_layout.height = self.content_size.height;
        self.item_layout.width = self.item_size.width;
        self.item_layout.height = self.item_size.height;
    }

    pub fn compute_item_position(&mut self) {
        self.item_layout.x =
            self.target.x + self.alignment.horizontal.shift_factor() * self.item_size.width;
        self.item_layout.y =
            self.target.y + self.alignment.vertical.shift_factor() * self.item_size.height;
    }

    pub fn compute_content_position(&mut self) {
        self.content_layout.x =
            self.item_layout.x + (self.item_layout.width - self.content_layout.width) * 0.5;
        self.content_layout.y =
            self.item_layout.y + (self.item_layout.height - self.content_layout.height) * 0.5;
    }
}
