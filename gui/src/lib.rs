#[macro_use]
extern crate bitflags;

pub mod alignment;
// pub mod label;
pub mod mouse;
pub mod widget_operation;
pub mod widget_geometry;
pub mod widget_model;
pub mod widget_state;

pub mod font;
pub mod gui;
pub mod widget_data;
pub mod root;
pub mod widget;
pub mod label;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
