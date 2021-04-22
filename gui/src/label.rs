use std::cell::{RefCell, Cell, Ref};
use std::rc::Rc;

use raylib::color::Color;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::*;


use crate::widget::Widget;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};


use crate::widget_operation::{RenderableWidget, DirtyFlags, WidgetOp};
use crate::gui::{Gui, GuiData};
use uuid::Uuid;
use generational_arena::Index;
use vec_tree::VecTree;
use crate::size::{Size, CachedSize};
use crate::text_style::TextStyle;
use crate::fill::Fill::Enabled;

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

    pub fn clear_text(&self) -> &LabelPar {
        let has_no_text = RefCell::borrow(&self.text).as_ref().is_none();
        if !has_no_text {
            self.text.replace(None);
            //todo dirty
            self.widget_data.set_dirty_flag(DirtyFlags::SIZE);
        }
        self
    }

    pub fn set_text(&self, text: String) -> &LabelPar {
        let borrowed_text = self.text.borrow();
        let current_text = borrowed_text.as_ref();
        if let Some(txt) = current_text {
            if text.eq(txt) {
                return self;
            }
        }
        self.widget_data.set_dirty_flag(DirtyFlags::SIZE);
        self.text.replace(Some(text));
        self
    }
}


impl SizeableWidget for LabelPar {
    fn update_preferred_size(&self, gui: &Gui) {
        if self.widget_data().dirty_flag_clean(DirtyFlags::PREFERRED_SIZE) {
            return
        }

        let padding = self.widget_data.padding();

        let text_size = match self.text.borrow().as_ref() {
            None => Size::empty(),
            Some(text) => {
                let borrowed_text_style = self.widget_data.state.text_style.borrow();
                let text_style = borrowed_text_style.as_ref();
                match text_style {
                    None => Size::empty(),
                    Some(t) => {
                        gui.measure_text(text, t)
                    }
                }
            }
        }.with_padding(&padding);

        let mut requested = self.widget_data.geometry.requested_size.get();
        requested.replace_empty_dimensions(&text_size).min_mut(&text_size);

        self.widget_data.geometry.preferred_size.replace(requested);
        self.widget_data.invalidate_content_size(gui)
    }

    fn update_content_size(&self, gui: &Gui, available_space: &Size) {
        let mut content_cache = self.widget_data.geometry.content_size.borrow_mut();
        let clean_flag =  self.widget_data().dirty_flag_clean(DirtyFlags::PREFERRED_SIZE);
        let cache_valid = available_space.eq(content_cache.reference());
        if clean_flag && cache_valid {
            return
        }

        content_cache.set_reference(available_space.to_owned());

        let mut content_size = self.widget_data.geometry.preferred_size.clone().into_inner();

        if let Enabled(_) = self.widget_data.geometry.fill_width.get() {
            content_size.set_width(available_space.width())
        }
        if let Enabled(_) = self.widget_data.geometry.fill_height.get() {
            content_size.set_height(available_space.height())
        }

        content_cache.set_size(content_size);
        self.widget_data.invalidate_size(gui);
    }
}

impl RenderableWidget for LabelPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, position:Vector2) {
        {
            let widget_layout = self.widget_data.geometry.widget_layout.clone().into_inner();
            if let Some(background) = &self.widget_data.state.background {
                background.draw(d, &widget_layout)
            }
            if let Some(border) = &self.widget_data.state.border {
                border.draw(d, &widget_layout)
            }
            self.widget_data.geometry.widget_layout.replace(widget_layout);
        }


        if let Some(text) = self.text.borrow().as_ref() {
            let content_layout = self.widget_data.geometry.content_layout.borrow();
            let position = Vector2 {
                x: content_layout.x,
                y: content_layout.y,
            };
            if let Some(text_style) = self.widget_data.state.text_style.borrow().as_ref() {
                gui.draw_text(d, text.as_str(), text_style, &position)
            }
        }
    }
}
