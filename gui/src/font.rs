use std::rc::Rc;

use raylib::prelude::*;

use crate::widget_operation::Size;
use std::cell::RefCell;
use crate::gui::GuiData;

#[derive(Clone)]
pub struct FontInfo {
    pub font: Rc<Font>,
    pub size: i32,
}

impl FontInfo {
    /// Measure the provided text with this font information
    /// # Arguments
    ///
    /// * `text` - The text to measure
    /// * `spacing` - The spacing to use for the measurement
    ///
    pub fn measure_text(&self, text: &str, spacing: f32) -> Size {
        let size = measure_text_ex(&self.font.as_ref(), text, self.size as f32, spacing);
        Size {
            width: size.x,
            height: size.y,
        }
    }

    /// Draw the provided text with this font information
    /// # Arguments
    ///
    /// * `text` - the text to draw
    /// * `position` - the position where the text must be drawn
    /// * `color` - the color of the text
    ///
    pub fn draw_text(
        &self,
        d: &mut RaylibDrawHandle<'_>,
        text: &str,
        position: &Vector2,
        spacing: f32,
        color: Color,
    ) {
        d.draw_text_ex(
            &self.font.as_ref(),
            text,
            position,
            self.size as f32,
            spacing,
            color,
        )
    }
}
