use std::cell::{RefCell, Cell};

use raylib::prelude::*;
use std::ops::Deref;

use crate::widget_data::{WidgetData};


use crate::widget_operation::{RenderableWidget, WidgetSpecific, WidgetDataProvider};
use crate::gui::{Gui};
use crate::size::{Size};

pub struct LabelPar {
    widget_data: WidgetData,
    text: RefCell<Option<String>>,
    text_size: Cell<Size>
}


impl WidgetDataProvider for LabelPar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }
    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}

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
            self.widget_data.invalidate_preferred_size(gui)
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
        self.widget_data.invalidate_preferred_size(gui);
        self.text.replace(Some(text.to_owned()));
        self
    }

    fn measure_text(&self) -> Size {
        match self.text.borrow().as_ref() {
            None => Size::empty(),
            Some(text) => {
                let borrowed_text_style = self.widget_data.state.text_style.borrow();
                match borrowed_text_style.deref() {
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
    fn compute_size(&self, _gui: &Gui) -> Size {
        let padding = self.widget_data.model.padding.get();
        let text_size = self.measure_text();

        self.text_size.set(text_size);

        let text_size_with_padding = text_size.with_padding(&padding).width_border(3.0);

        let mut preferred = self.widget_data.model.preferred_size.get();

        preferred.replace_empty_dimensions_and_max(&text_size_with_padding);
        preferred
    }

    fn compute_child_content_size(&self, _gui: &Gui, _available_space: Size) {}

    fn compute_child_positions(&self, _gui: &Gui) {
    }
}

impl RenderableWidget for LabelPar {
    fn render(&self, _gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2) {


        self.widget_data.render_background_and_border(d,&offset);



        if let Some(text) = self.text.borrow().as_ref() {
            let content_layout = self.widget_data.geometry.content_layout.get();
            let text_size = self.text_size.get();
            let position = Vector2 {
                x: content_layout.x + offset.x + (content_layout.width - text_size.width())*0.5,
                y: content_layout.y + offset.y + (content_layout.height - text_size.height())*0.5,
            };

            let borrowed_text_style = self.widget_data.state.text_style.borrow();
            let text_style = borrowed_text_style.as_deref();
            if let Some(text_style) = text_style {
                text_style.draw_text(d, text, &position)
            }
        }
    }
}
