use raylib::prelude::{RaylibDrawHandle, Rectangle};

/// A background
pub trait BackgroundRenderer {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}


pub enum Background {
    Empty,
}


impl BackgroundRenderer for Background {

    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle) {
        match self {
            Background::Empty => {}
        }
    }
}