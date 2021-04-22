use raylib::prelude::{RaylibDrawHandle, Rectangle};

/// A background
pub trait Background {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>, layout: &Rectangle);
}

