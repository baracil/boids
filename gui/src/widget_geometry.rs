use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use std::cell::{Cell, RefCell};
use crate::size::{Size, CachedSize};
use crate::fill::Fill;
use crate::fill::Fill::Disabled;

pub struct WidgetGeometry {
    pub fill_height: Cell<Fill>,
    pub fill_width: Cell<Fill>,

    /// Preferred size
    pub preferred_size: RefCell<Size>,
    /// Size of the content base on the available space provided
    pub content_size: RefCell<CachedSize>,

    /// The full size of the widget (content + borders)
    pub widget_size: RefCell<Size>,

    /// The position and size of the content in absolute coordinate
    pub content_layout: RefCell<Rectangle>,
    /// The position and size of the widget in absolute coordinate
    pub widget_layout: RefCell<Rectangle>,    //layout of the background

    /// The requested size
    pub requested_size: Cell<Size>, //requested size of the node

    /// the requested position where the node should be drawn
    pub target: Cell<Vector2>,
    pub absolute_coordinate_y: Cell<bool>,
    pub absolute_coordinate_x: Cell<bool>,
    /// alignment to the target
    pub alignment: Cell<Alignment>,
}

impl WidgetGeometry {
    pub(crate) fn new() -> Self {
        Self {
            target: Cell::new(Default::default()),
            absolute_coordinate_y: Cell::new(true),
            absolute_coordinate_x: Cell::new(true),
            alignment: Cell::new(Default::default()),
            widget_layout: RefCell::new(Default::default()),
            content_layout: RefCell::new(Default::default()),
            content_size: RefCell::new(Default::default()),
            widget_size: RefCell::new(Default::default()),
            requested_size: Cell::new(Default::default()),
            preferred_size: RefCell::new(Default::default()),
            fill_height: Cell::new(Disabled),
            fill_width: Cell::new(Disabled),
        }
    }

    fn copy_size(source: &Size, target: &RefCell<Rectangle>) {
        let mut target_layout = target.borrow_mut();
        target_layout.width = source.width();
        target_layout.height = source.height();
    }

    pub fn copy_size_to_layout(&self) {
        WidgetGeometry::copy_size(&self.content_size.borrow().size(), &self.content_layout);
        WidgetGeometry::copy_size(&self.widget_size.borrow(), &self.widget_layout);
    }

    fn compute_position(target: f32, absolute: bool, available: f32) -> f32 {
        if absolute {
            return target;
        }
        target * 0.01 * available
    }

    pub fn compute_item_position(&mut self, available_size: &Size) {
        let target = self.target.get();
        let alignment = self.alignment.get();
        let widget_size = self.widget_size.borrow();

        let x = WidgetGeometry::compute_position(target.x, self.absolute_coordinate_x.get(), available_size.width());
        let y = WidgetGeometry::compute_position(target.y, self.absolute_coordinate_y.get(), available_size.height());

        let wx = x + alignment.horizontal.shift_factor() * widget_size.width();
        let wy = y + alignment.vertical.shift_factor() * widget_size.height();

        let mut layout = self.widget_layout.borrow_mut();
        layout.x = wx;
        layout.y = wy;
    }

    pub fn compute_content_position(&mut self) {
        let widget_layout = self.widget_layout.borrow_mut();
        let mut content_layout = self.content_layout.borrow_mut();

        let lx = widget_layout.x + (widget_layout.width - content_layout.width) * 0.5;
        let ly = widget_layout.y + (widget_layout.height - content_layout.height) * 0.5;

        content_layout.x = lx;
        content_layout.y = ly;
    }
}
