use std::cell::{Cell, RefCell};
use crate::widget_data::WidgetData;
use crate::gui::Gui;
use crate::widget_operation::{WidgetDataProvider, WidgetSpecific, RenderableWidget};
use crate::size::Size;
use raylib::prelude::*;
use std::rc::Rc;
use crate::text_style::TextStyle;

pub struct SliderPar {
    widget_data: WidgetData,
    value: Cell<f32>,
    value_min: Cell<f32>,
    value_max: Cell<f32>,
    value_text:RefCell<String>,
    value_text_size: Cell<Size>,
}


const SLIDER_BAR_HEIGHT: f32 = 20.0;
const SLIDER_BAR_WIDTH: f32 = 100.0;
const SLIDER_BAR_THICKNESS: f32 = 1.0;
const SLIDER_BAR_COLOR: Color = Color::GRAY;


const SLIDER_CURSOR_SPACING: f32 = 2.0;
const SLIDER_CURSOR_HEIGHT: f32 = 30.0;
const SLIDER_CURSOR_WIDTH: f32 = 5.0;
const SLIDER_CURSOR_THICKNESS: f32 = 1.0;
const SLIDER_CURSOR_COLOR: Color = Color::BLACK;

impl SliderPar {
    pub fn new() -> Self {
        Self {
            widget_data: WidgetData::new(),
            value: Cell::new(50.0),
            value_min: Cell::new(0.0),
            value_max: Cell::new(100.0),
            value_text: RefCell::new("".to_string()),
            value_text_size: Cell::new(Size::empty()),
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value.get()
    }

    pub fn get_value_min(&self) -> f32 {
        self.value_min.get()
    }

    pub fn get_value_max(&self) -> f32 {
        self.value_max.get()
    }

    pub fn set_value(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value.set(value);
        self.widget_data.invalidate_preferred_size(gui);
        self
    }

    pub fn set_value_min(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value_min.set(value);
        self.widget_data.invalidate_preferred_size(gui);
        self
    }

    pub fn set_value_max(&self, gui: &Gui, value: f32) -> &SliderPar {
        self.value_max.set(value);
        self.widget_data.invalidate_preferred_size(gui);
        self
    }


    fn format_value(&self, value:f32) -> String {
        format!("{:5.1}", value)
    }

    fn measure_value(&self, formatted_value: &str) -> Size {
        let borrowed_text_style = &self.widget_data.state.text_style.borrow();

        match borrowed_text_style.as_ref() {
            None => Size::empty(),
            Some(ts) => {
                ts.measure_text(formatted_value)
            }
        }
    }
}

impl WidgetDataProvider for SliderPar {
    fn widget_data(&self) -> &WidgetData {
        &self.widget_data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.widget_data
    }
}

impl WidgetSpecific for SliderPar {
    fn compute_size(&self, gui: &Gui) -> Size {
        let formatted_value = self.format_value(self.value.get());
        let text_size = self.measure_value(&formatted_value);
        self.value_text.replace(formatted_value);
        self.value_text_size.set(text_size);
        let bar_size = Size::new(SLIDER_BAR_WIDTH, SLIDER_BAR_HEIGHT);
        return text_size.max(&bar_size).with_padding(&self.widget_data.model.padding.get());
    }

    fn compute_child_content_size(&self, gui: &Gui, available_size: Size) {}

    fn compute_child_positions(&self, gui: &Gui) {}
}

impl RenderableWidget for SliderPar {
    fn render(&self, gui: &Gui, d: &mut RaylibDrawHandle<'_>, offset: &Vector2) {
        self.widget_data.render_background_and_border(d, offset);

        let mut content_layout = self.widget_data.geometry.content_layout.get();

        content_layout.x += offset.x;
        content_layout.y += offset.y;

        d.draw_rectangle_rec(content_layout, SLIDER_BAR_COLOR);


        {
            let value = self.value.get();
            let value_min = self.value_min.get();
            let value_max = self.value_max.get();
            let width_minus_cursor_width = content_layout.width - SLIDER_CURSOR_WIDTH;
            let cursor_position_x = content_layout.x + width_minus_cursor_width * (value-value_min)/(value_max-value_min);
            let cursor_position_y = content_layout.y + SLIDER_CURSOR_SPACING;

            let rectangle = Rectangle::new(cursor_position_x, cursor_position_y,SLIDER_CURSOR_WIDTH, content_layout.height - 2.0*SLIDER_CURSOR_SPACING );

            d.draw_rectangle_rec(rectangle,Color::GREEN)

        }

        {
            let borrowed_text_style = &self.widget_data.state.text_style.borrow();
            if let Some(ts) = borrowed_text_style.as_ref() {
                let text_size = self.value_text_size.get();
                let mut position = Vector2::new(content_layout.x, content_layout.y);
                position.x += (content_layout.width - text_size.width())*0.5;
                position.y += (content_layout.height - text_size.height())*0.5;

                let borrowed_text = self.value_text.borrow();
                ts.draw_text(d,borrowed_text.as_str(),&position);
            }
        }





    }

}
