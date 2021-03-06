use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn subtract(&mut self, rhs: &Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    pub fn add(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn add_scaled(&mut self, rhs: &Vector, scale: f32) {
        self.x += rhs.x * scale;
        self.y += rhs.y * scale;
    }

    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }

    pub fn norm(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn hypot(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn set_random(&mut self, norm:f32) {
        let angle: f32 = rand::random::<f32>() * PI * 2.0;
        self.x = norm * angle.cos();
        self.y = norm * angle.sin();
    }

}
