use std::cell::{RefCell, Cell};

use raylib::prelude::*;
use std::ops::Deref;

use crate::widget_data::{WidgetData};


use crate::widget_operation::{RenderableWidget, WidgetSpecific};
use crate::gui::{Gui};
use crate::size::{Size};

pub struct LabelPar {
    widget_data: WidgetData,
    text: RefCell<Option<String>>,
    text_size: Cell<Size>
}

impl Deref for LabelPar {
    type Target = WidgetData;

    fn deref(&self) -> &Self::Target {
        &self.widget_data
    }
}

// impl WidgetDataProvider for LabelPar {
//     fn widget_data(&self) -> &WidgetData {
//         &self.widget_data
//     }
//     fn widget_data_mut(&mut self) -> &mut WidgetData {
//         &mut self.widget_data
//     }
// }

impl LabelPar {
    pub fn new() -> Self {
        Self {
            widget_data: WidgetData::new(),
            text: RefCell::new(None),
            text_size: Cell::new(Size::empty())
        }
    }

    pub fn clear_text(&self, gui: &Gui) -> &LabelPar {
        let has_no_text = RefCell::borrow(&self.text).as_ref().is_none();
        if !has_no_text {
            self.text.replace(None);
            self.invalidate_preferred_size(gui)
        }
        self
    }

    pub fn set_text(&self, gui: &Gui, text: &str) -> &LabelPar {
        let is_text_valid = {
            let current_text = self.text.borrow().as_ref().cloned();
            Some(text.to_owned()).eq(&current_text)
        };
        if is_text_valid  {
                return self;
        }
        self.invalidate_preferred_size(gui);
        self.text.replace(Some(text.to_owned()));
        self
    }

    fn measure_text(&self) -> Size {
        match self.text.borrow().as_ref() {
            None => Size::empty(),
            Some(text) => {
                match self.get_text_style() {
                    None => Size::empty(),
                    Some(text_style) => {
                        text_style.measure_text(text)
                    }
                }
            }
        }
    }
}


impl WidgetSpecific for LabelPar {

    fn get_widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn get_widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }

    fn compute_size(&self, _gui: &Gui) -> Size {
        let padding = self.get_padding();
        let text_size = self.measure_text();

        self.text_size.set(text_size);

        let text_size_with_padding = text_size.with_padding(&padding).width_border(3.0);

        let mut preferred = self.get_preferred_size();

        preferred.replace_empty_dimensions_and_max(&text_size_with_padding);
        preferred
    }

    fn compute_child_content_size(&self, _gui: &Gui, _available_space: Size) {}

    fn compute_child_positions(&self, _gui: &Gui) {
    }
}

impl RenderableWidget for LabelPar {
    fn render(&self, _gui: &Gui, d: &mut impl RaylibDraw, offset: &Vector2) {

        self.render_background_and_border(d,&offset);

        if let Some(text) = self.text.borrow().as_ref() {
            let content_layout = self.get_content_layout();
            let text_size = self.text_size.get();
            let position = Vector2 {
                x: content_layout.x + offset.x + (content_layout.width - text_size.width())*0.5,
                y: content_layout.y + offset.y + (content_layout.height - text_size.height())*0.5,
            };

            if let Some(text_style) = self.get_text_style() {
                text_style.draw_text(d, text, &position)
            }
        }

    }

}
