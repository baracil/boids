use crate::padding::Padding;
use std::cell::{Cell, RefCell};
use crate::fill::Fill;
use crate::size::Size;
use crate::fill::Fill::Disabled;
use raylib::math::{Rectangle, Vector2};
use crate::alignment::Alignment;

pub struct WidgetModel {

    pub fill_height: Cell<Fill>,
    pub fill_width: Cell<Fill>,

    pub position: Cell<Vector2>,
    pub absolute_coordinate_y: Cell<bool>,
    pub absolute_coordinate_x: Cell<bool>,

    pub text_style_name: RefCell<String>,
    pub back_style_name: RefCell<String>,
    pub border_style_name: RefCell<String>,

    pub user_preferred_size: Cell<Size>,

    pub focusable: Cell<bool>,
    pub clickable: Cell<bool>,
    pub disable: Cell<bool>,

    pub padding: Cell<Padding>,

    pub action_id: RefCell<Option<String>>,

    /// alignment to the target
    pub alignment: Cell<Alignment>,

}

impl WidgetModel {
    pub(crate) fn new() -> Self {
        Self {
            position:Cell::new(Default::default()),
            absolute_coordinate_y: Cell::new(true),
            absolute_coordinate_x: Cell::new(true),
            fill_height: Cell::new(Disabled),
            fill_width: Cell::new(Disabled),
            user_preferred_size: Cell::new(Default::default()),
            text_style_name: RefCell::new("default".to_string()),
            back_style_name: RefCell::new("default".to_string()),
            border_style_name: RefCell::new("default".to_string()),
            alignment: Cell::new(Default::default()),
            focusable: Cell::new(false),
            clickable: Cell::new(false),
            disable: Cell::new(false),
            padding: Cell::new(Padding::none()),
            action_id: RefCell::new(None),
        }
    }
}
