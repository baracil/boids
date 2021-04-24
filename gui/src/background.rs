use raylib::prelude::*;

/// A background
pub trait BackgroundRenderer {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle, hoovered:bool);
}


pub enum Background {
    Empty,
    Solid {
        color:Color,
        hoovered_color:Color,
    }
}


impl BackgroundRenderer for Background {

    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle, hoovered:bool) {
        match self {
            Background::Empty => {}
            Background::Solid {color,hoovered_color} => render_solid_background(d,layout, if hoovered {hoovered_color} else {color})
        }
    }
}

fn render_solid_background(d: &mut RaylibDrawHandle<'_>, layout: &Rectangle, color: &Color) {
    d.draw_rectangle_rec(layout,color)
}