use raylib::prelude::{RaylibDrawHandle, Rectangle};

/// A border
pub trait BorderRenderer {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}

pub enum Border {
    Empty,
}


impl BorderRenderer for Border {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle) {
        match self {
            Border::Empty => {}
        }
    }
}
