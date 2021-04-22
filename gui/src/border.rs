use raylib::prelude::{RaylibDrawHandle, Rectangle};

/// A border
pub trait Border {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}
