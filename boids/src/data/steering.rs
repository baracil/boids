use crate::data::vector::Vector;

pub struct Steering {
    pub separation: Vector,
    pub alignment: Vector,
    pub cohesion: Vector,
}

impl Steering {
    pub fn new() -> Self {
        Steering {
            separation: Vector::new(),
            alignment: Vector::new(),
            cohesion: Vector::new(),
        }
    }

    pub fn clear(&mut self) {
        self.separation.clear();
        self.alignment.clear();
        self.cohesion.clear();
    }
}
