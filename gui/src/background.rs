use raylib::prelude::*;

/// A background
pub trait BackgroundRenderer {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}


pub enum Background {
    Empty,
    Solid {
        color:Color
    }
}


impl BackgroundRenderer for Background {

    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle) {
        match self {
            Background::Empty => {}
            Background::Solid {color} => render_solid_background(d,layout, color)
        }
    }
}

fn render_solid_background(d: &mut RaylibDrawHandle<'_>, layout: &Rectangle, color: &Color) {
    d.draw_rectangle_rec(layout,color)
}