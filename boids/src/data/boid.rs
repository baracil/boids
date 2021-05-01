use std::f32::consts::PI;

use crate::data::vector::Vector;

#[derive(Copy, Clone)]
pub struct Boid {
    pub position: Vector,
    pub velocity: Vector,
    speed: f32,
}

impl Boid {
    pub fn new() -> Self {
        Boid {
            position: Vector::new(),
            velocity: Vector::new(),
            speed: 0.0,
        }
    }

    pub fn update_speed(&mut self) {
        self.speed = self.velocity.x.hypot(self.velocity.y);
    }

    pub fn update_position(&mut self, dt: f32) {
        self.position.add_scaled(&self.velocity, dt);
    }

    pub fn clamp_speed(&mut self, min_speed: f32, max_speed: f32) {
        if self.speed > max_speed {
            self.velocity.scale(max_speed / self.speed);
            self.speed = max_speed;
        }

        if self.speed <= 1e-6 {
            let v: f32 = rand::random::<f32>() * PI * 2.0;
            self.velocity.x = min_speed * v.cos();
            self.velocity.y = min_speed * v.sin();
            self.speed = min_speed;
        } else if self.speed < min_speed {
            self.velocity.scale(min_speed / self.speed);
            self.speed = min_speed;
        }
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
}
