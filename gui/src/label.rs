use std::cell::RefCell;
use std::rc::Rc;

use raylib::color::Color;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::prelude::*;

use tree::tree::{TreeData, TreeDataProvider};


use crate::font::FontInfo;

use crate::widget::Widget;
use crate::widget_data::{SizeableWidget, WidgetDataProvider, WidgetData};


use crate::widget_operation::{RenderableWidget, Size, DirtyFlags};
use crate::gui::{InnerGui, GuiData};
use uuid::Uuid;


pub struct LabelPar {
    tree_data: Rc<RefCell<TreeData<Widget>>>,
    widget_data: WidgetData,
    text: Option<String>,
    font_id: Option<Uuid>,
    spacing: f32,
    //todo use style to define this value
    color: Color, //todo use style to define this value
}

impl TreeDataProvider<Widget> for LabelPar {
    fn tree_data(&self) -> Rc<RefCell<TreeData<Widget>>> {
        self.tree_data.clone()
    }
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
    pub fn new(gui:Rc<RefCell<GuiData>>) -> Self {
        Self {
            tree_data: Rc::new(RefCell::new(TreeData::new())),
            widget_data: WidgetData::new(gui),
            text: None,
            font_id: None,
            spacing: 0.0,
            color: Color::BLACK,
        }
    }

    pub fn set_font_id(&mut self, font_id:Uuid) -> &mut LabelPar {
        self.widget_data.set_dirty_flag(DirtyFlags::SIZE);
        self.font_id = Some(font_id);
        self
    }

    pub fn clear_text(&mut self) -> &mut LabelPar {
        if let Some(_) = self.text {
            self.widget_data.set_dirty_flag(DirtyFlags::SIZE);
            self.text = None;
        }
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut LabelPar {
        if let Some(txt) = &self.text {
            if text.eq(txt) {
                return self;
            }
        }
        self.widget_data.set_dirty_flag(DirtyFlags::SIZE);
        self.text = Some(text);
        self
    }
}


impl SizeableWidget for LabelPar {
    fn compute_content_size(&self) -> Size {
        let text = match &self.text {
            None => "",
            Some(text) => text.as_str(),
        };

        match &self.font_id {
            None => Size::empty(),
            Some(f) => self.widget_data.measure_text(f, text, self.spacing)
        }
    }
}

impl RenderableWidget for LabelPar {
    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(background) = &self.widget_data.state.background {
            background.draw(d, &self.widget_data.geometry.item_layout)
        }
        if let Some(border) = &self.widget_data.state.border {
            border.draw(d, &self.widget_data.geometry.item_layout)
        }

        d.draw_rectangle_rec(self.widget_data.geometry.item_layout, Color::GREEN);

        if let Some(text) = &self.text {
            let position = Vector2 {
                x: self.widget_data.geometry.content_layout.x,
                y: self.widget_data.geometry.content_layout.y,
            };
            if let Some(font_id) = &self.font_id {
                self.widget_data.draw_text(d,font_id,text.as_str(), &position, self.spacing, self.color)
            }
        }
    }
}
