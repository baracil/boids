use raylib::prelude::{Color, RaylibDrawHandle, Vector2};
use std::rc::Rc;
use crate::size::Size;
use crate::font::FontInfo;

#[derive(Clone)]
pub struct TextStyle {
    font:Rc<FontInfo>,
    color:Color,
    spacing:f32,
}

impl TextStyle {
    pub fn default(font:Rc<FontInfo>) -> Self {
        Self{font:font.clone(), color:Color::BLACK, spacing:0.0}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }

    pub fn measure_text(&self, text: &str) -> Size {
        self.font.measure_text(text, self.spacing)
    }

    pub fn draw_text(&self, d: &mut RaylibDrawHandle, text: &str, position: &Vector2) {
        self.font.draw_text(d,text, position,self.spacing,self.color);
    }


}