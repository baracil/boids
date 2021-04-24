use raylib::math::{Rectangle, Vector2};

use crate::alignment::Alignment;
use std::cell::{Cell, RefCell};
use crate::size::{Size, CachedSize};
use crate::fill::Fill;
use crate::fill::Fill::Disabled;
use crate::gui::Gui;

/// Geometry of the widget obtained from the information contained in the model
pub struct WidgetGeometry {
    /// Preferred size
    pub computed_size: Cell<Size>,

    /// Size of the content base on the available space provided
    pub widget_size: RefCell<CachedSize>,

    /// The position and size of the widget in relative coordinate to the parent
    pub widget_layout: RefCell<Rectangle>,
    /// The position and size of the content (same as widget_layout but without padding)
    pub content_layout: RefCell<Rectangle>,

}

impl WidgetGeometry {
    pub(crate) fn new() -> Self {
        Self {
            content_layout: RefCell::new(Default::default()),
            widget_size: RefCell::new(Default::default()),
            widget_layout: RefCell::new(Default::default()),
            computed_size: Cell::new(Default::default()),
        }
    }

    pub(crate) fn copy_size(source: &Size, target: &RefCell<Rectangle>) {
        let mut target_layout = target.borrow_mut();
        target_layout.width = source.width();
        target_layout.height = source.height();
    }



}
