use raylib::prelude::*;
use std::vec::Vec;
use std::thread::current;

//all measurements are in SI
const BIRD_SIZE:f32 = 0.2; //20cm
const SEPARATION_FACTOR:f32 = 0.1;
const COHESION_FACTOR:f32 = 0.1;
const ALIGNMENT_FACTOR:f32 = 0.1;
// const fn cos_max_angle() -> f32 {
//     let angle = std::f32::consts::PI * 5. / 9.;
//     angle.cos()
// }


#[derive(Copy,Clone)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {

    pub fn new() -> Self {
        Vector{x:0.0,y:0.0}
    }

    pub fn subtract(&mut self, rhs: &Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

    pub fn add(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    pub fn add_scaled(&mut self, rhs: &Vector, scale:f32) {
        self.x += rhs.x*scale;
        self.y += rhs.y*scale;
    }

    pub fn scale(&mut self, scale:f32) {
        self.x *= scale;
        self.y *= scale;
    }

    pub fn hypot(&self) -> f32 {
        self.x.hypot(self.y)
    }
}

struct Steering {
    separation:Vector,
    alignment:Vector,
    cohesion:Vector
}

impl Steering {
    pub fn new() -> Self {
        Steering{
            separation: Vector::new(),
            alignment: Vector::new(),
            cohesion: Vector::new()
        }
    }
}

#[derive(Copy,Clone)]
struct Boid {
    position: Vector,
    velocity: Vector,
    speed: f32,
}

impl Boid {
    pub fn update_position(&mut self, dt:f32) {
        self.position.add_scaled(&self.velocity,dt);
    }
}

struct World {
    cos_max_angle:f32,
    current_is_one: bool,
    boids1 : Vec<Boid>,
    boids2 : Vec<Boid>,
}

impl World {

    pub fn compute(&mut self,dt:f32) {
        let mut current;
        let mut next;
        if self.current_is_one {
            current = &self.boids1;
            next = &self.boids2;
        } else {
            current = &self.boids2;
            next = &self.boids1;
        }

        // self.compute_new_boid_positions(current, next.as_mut_slice(), dt);

        self.current_is_one = !self.current_is_one;
    }

    pub fn compute_new_boid_positions(&mut self, current : &Vec<Boid>, next: &mut[Boid],   dt:f32) {
        let mut steering = Steering::new();

        for (i,boid) in current.iter().enumerate() {

            self.compute_steering(*boid, current.as_slice(), &mut steering);
            // next[i].position = boid.position;
            // next[i].position.add_scaled(boit.velocity,dt);
            //todo use also steering
        }
    }

    fn compute_steering(&mut self, reference: Boid, current : &[Boid], steering: &mut Steering) {
        let mut buffer= Vector{x:0.,y:0.};
        steering.separation.x=0.0;
        steering.separation.y=0.0;
        steering.alignment.x=0.0;
        steering.alignment.y=0.0;
        steering.cohesion.x=0.0;
        steering.cohesion.y=0.0;

        let mut nb_neighbour = 0;
        for boid in current {
            let visible = self.compute_separation(reference, *boid, &mut buffer);
            if visible {
                nb_neighbour+=1;
                steering.separation.add(&buffer);
                steering.alignment.add_scaled(&boid.velocity, 1./boid.speed);
                steering.cohesion.add(&boid.position);
            }
        }
        if nb_neighbour>0 {
            steering.alignment.scale(1./(nb_neighbour as f32));
            steering.cohesion.scale(1./(nb_neighbour as f32));
        }
    }


    fn compute_separation(&mut self, reference: Boid, other: Boid, separation:&mut Vector) -> bool {
        *separation = reference.position;
        separation.subtract(&other.position);

        let distance = separation.hypot();
        let prod = (separation.x * reference.velocity.x + separation.y * reference.velocity.y) / (distance * reference.speed);

        if prod<self.cos_max_angle {
            return false;
        }

        let factor = (1./distance.max(BIRD_SIZE)).powi(2);
        separation.x /=factor;
        separation.y /=factor;

        true

    }


}








fn draw_birds() {}

fn compute_bird_positions() {}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .msaa_4x()
        .resizable()
        .vsync()
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        draw_birds();
        compute_bird_positions();
    }
}
