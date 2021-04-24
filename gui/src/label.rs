use std::cell::{RefCell, Ref};

use raylib::prelude::*;
use std::ops::Deref;

use crate::widget_data::{WidgetData};


use crate::widget_operation::{RenderableWidget, DirtyFlags, WidgetOp, LayoutableWidget, SizeComputer, WidgetDataProvider};
use crate::gui::{Gui};
use crate::size::{Size};
use crate::fill::Fill::Enabled;
use crate::background::BackgroundRenderer;
use crate::border::BorderRenderer;

pub struct LabelPar {
    widget_data: WidgetData,
    text: RefCell<Option<String>>,
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

    pub fn set_text(&self, gui: &Gui, text: String) -> &LabelPar {
        let borrowed_text = self.text.borrow();
        let current_text = borrowed_text.as_ref();
        if let Some(txt) = current_text {
            if text.eq(txt) {
                return self;
            }
        }
        self.widget_data.invalidate_preferred_size(gui);
        self.text.replace(Some(text));
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


impl SizeComputer for LabelPar {
    fn compute_size(&self, gui: &Gui) -> Size {
        let padding = self.widget_data.model.padding.get();
        let text_size = self.measure_text().with_padding(&padding);

        let mut preferred = self.widget_data.model.preferred_size.get();

        preferred.replace_empty_dimensions_and_max(&text_size);
        preferred
    }

    fn compute_child_content_size(&self, gui: &Gui, available_space: Size) {}

    fn compute_child_positions(&self, gui: &Gui) {
    }
}

impl RenderableWidget for LabelPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2, available_space: &Size) {


        self.widget_data.render_background_and_border(d,&offset);


        if let Some(text) = self.text.borrow().as_ref() {
            let content_layout = self.widget_data.geometry.content_layout.borrow();
            let position = Vector2 {
                x: content_layout.x + offset.x,
                y: content_layout.y + offset.y,
            };


            let borrowed_text_style = self.widget_data.state.text_style.borrow();
            let text_style = borrowed_text_style.as_deref();
            if let Some(text_style) = text_style {
                text_style.draw_text(d, text, &position)
            }
        }
    }
}
