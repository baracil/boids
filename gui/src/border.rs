use raylib::prelude::*;

/// A border
pub trait BorderRenderer {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}

pub enum Border {
    Empty,
    Line {
        color:Color,
        thickness:f32
    }
}


impl BorderRenderer for Border {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle) {
        match self {
            Border::Empty => {}
            Border::Line {color, thickness:thickness } => render_line_border(d, layout, color, *thickness)
        }
    }
}

fn render_line_border(d: &mut RaylibDrawHandle, layout: &Rectangle, color: &Color, thickness: f32) {
    d.draw_rectangle_lines_ex(layout, thickness as i32, color)
}
