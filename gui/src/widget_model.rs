use crate::padding::Padding;
use std::cell::Cell;

pub struct WidgetModel {
    pub text_style_name: String,
    pub back_style_name: String,
    pub border_style_name: String,

    pub focusable: bool,
    pub clickable: bool,

    pub armed: bool,
    pub hoovered: bool,
    pub disable: bool,

    pub padding: Cell<Padding>,

    pub action_id: Option<String>,
}

impl WidgetModel {
    pub(crate) fn new() -> Self {
        Self {
            text_style_name: "default".to_string(),
            back_style_name: "default".to_string(),
            border_style_name: "default".to_string(),
            focusable: false,
            clickable: false,
            armed: false,
            hoovered: false,
            disable: false,
            padding: Cell::new(Padding::none()),
            action_id: None,
        }
    }
}
