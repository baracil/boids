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

    pub fn hypot(&self) -> f32 {
        self.x.hypot(self.y)
    }
}
