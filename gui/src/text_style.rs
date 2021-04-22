use raylib::prelude::Color;

#[derive(Clone)]
pub struct TextStyle {
    font_name:String,
    color:Color,
    spacing:f32,
}

impl TextStyle {
    pub fn new(font_name:String) -> Self {
        Self{font_name, color:Color::BLACK, spacing:0.0}
    }

    pub fn font_name(&self) -> &String {
        &self.font_name
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn spacing(&self) -> f32 {
        self.spacing
    }
}