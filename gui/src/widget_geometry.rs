use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use crate::widget_operation::Size;

pub struct WidgetGeometry {
    pub target: Vector2,
    //the requested position where the node should be drawn
    pub absolute_coordinate: bool,
    pub alignment: Alignment,
    //alignment to the target
    pub requested_size: Size, //requested size of the node

    pub fill_height_weight: u32,
    pub fill_height: bool,

    pub fill_width_weight: u32,
    pub fill_width: bool,

    pub content_size: Size,
    //size of the content
    pub item_size: Size,    // size of the item

    pub content_layout: Rectangle,
    //layout of the content
    pub item_layout: Rectangle,    //layout of the background
}

impl WidgetGeometry {
    pub(crate) fn new() -> Self {
        Self {
            target: Vector2::new(0.0, 0.0),
            absolute_coordinate: true,
            alignment: Alignment::new(),
            item_layout: Default::default(),
            content_layout: Default::default(),
            content_size: Size::empty(),
            item_size: Size::empty(),
            requested_size: Size::empty(),
            fill_height: false,
            fill_width: false,
            fill_height_weight: 1,
            fill_width_weight: 1,
        }
    }

    pub fn copy_size_to_layout(&mut self) {
        self.content_layout.width = self.content_size.width;
        self.content_layout.height = self.content_size.height;
        self.item_layout.width = self.item_size.width;
        self.item_layout.height = self.item_size.height;
    }

    pub fn compute_item_position(&mut self, available_size: &Size) {
        let x = if self.absolute_coordinate {
            self.target.x
        } else {
            available_size.width * self.target.x * 0.01
        };

        let y = if self.absolute_coordinate {
            self.target.y
        } else {
            available_size.height * self.target.y * 0.01
        };

        self.item_layout.x = x + self.alignment.horizontal.shift_factor() * self.item_size.width;
        self.item_layout.y = y + self.alignment.vertical.shift_factor() * self.item_size.height;
    }

    pub fn compute_content_position(&mut self) {
        self.content_layout.x =
            self.item_layout.x + (self.item_layout.width - self.content_layout.width) * 0.5;
        self.content_layout.y =
            self.item_layout.y + (self.item_layout.height - self.content_layout.height) * 0.5;
    }
}
