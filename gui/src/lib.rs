#[macro_use]
extern crate bitflags;

pub mod node;
pub mod alignment ;
pub mod button;
pub mod label;
pub mod mouse;
pub mod node_state;
pub mod node_model;
pub mod node_geometry;

pub mod private_node;
pub mod gui;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
