use crate::data::boid::Boid;
use rand::Rng;
use crate::data::steering::Steering;
use crate::data::vector::Vector;


const CONSTRAINT_STRENGTH: f32 = 0.1;
const DEAD_ANGLE: f32 = 0.0; // in degree

const DEFAULT_SEPARATION_FACTOR: f32 = 10.;
const DEFAULT_COHESION_FACTOR: f32 = 40.;
const DEFAULT_ALIGNMENT_FACTOR: f32 = 10.;

const DEFAULT_VISIBILITY_FACTOR: f32 = 5.0;

const DEFAULT_NB_BIRDS: usize = 500;

const RANDOM_FACTOR: f32 = 0.1;
const DEFAULT_WORLD_SIZE: f32 = 10.;
const DEFAULT_BIRD_SIZE: f32 = 0.2;
const DEFAULT_BIRD_MIN_SPEED: f32 = 5.0;
const DEFAULT_BIRD_MAX_SPEED: f32 = 16.0;

pub struct Parameters {
    pub bird_size: f32,
    pub min_bird_speed: f32,
    pub max_bird_speed: f32,
    pub visibility_radius: f32,
    pub cos_max_angle: f32,
    pub separation_factor: f32,
    pub cohesion_factor: f32,
    pub alignment_factor: f32,
}

impl Parameters {
    pub fn new() -> Self {
        Parameters {
            bird_size: DEFAULT_BIRD_SIZE,
            visibility_radius: DEFAULT_BIRD_SIZE * DEFAULT_VISIBILITY_FACTOR,
            cos_max_angle: (std::f32::consts::PI * (1. - DEAD_ANGLE / 180.)).cos(),
            cohesion_factor: DEFAULT_COHESION_FACTOR * 0.01,
            separation_factor: DEFAULT_SEPARATION_FACTOR * 0.01,
            alignment_factor: DEFAULT_ALIGNMENT_FACTOR * 0.01,
            min_bird_speed: DEFAULT_BIRD_MIN_SPEED,
            max_bird_speed: DEFAULT_BIRD_MAX_SPEED,
        }
    }
}


pub struct World {
    pub playfield_size: f32,
    pub parameters: Parameters,
    pub current: Vec<Boid>,
    pub next: Vec<Boid>,
}
impl World {
    pub fn new(nb_birds: usize, playfield: f32) -> Self {
        World {
            playfield_size: playfield,
            parameters: Parameters::new(),
            current: vec![Boid::new(); nb_birds],
            next: vec![Boid::new(); nb_birds],
        }
    }

    pub fn initialize(&mut self) {
        let mut rng = rand::thread_rng();
        for boid in self.current.iter_mut() {
            // let t: f32 = rng.gen::<f32>() * 2.0 * PI;
            // let u = rng.gen::<f32>() + rng.gen::<f32>();
            // let r: f32 = if u > 1.0 { 2.0 - u } else { u };

            // boid.position.x = r * t.cos() * self.playfield_size;
            // boid.position.y = r * t.sin() * self.playfield_size;
            boid.position.x = (rng.gen::<f32>() - 0.5) * self.playfield_size;
            boid.position.y = (rng.gen::<f32>() - 0.5) * self.playfield_size;
            boid.velocity.x = (rng.gen::<f32>() - 0.5) * self.playfield_size * 0.1;
            boid.velocity.y = (rng.gen::<f32>() - 0.5) * self.playfield_size * 0.1;
            boid.speed = boid.velocity.hypot();

            boid.clamp_speed(self.parameters.min_bird_speed, self.parameters.max_bird_speed);
        }
    }

    pub fn compute(&mut self, dt: f32) {
        let mut steering = Steering::new();
        let sum_of_factors = self.parameters.alignment_factor;
        let mut rng = rand::thread_rng();

        for (i, boid) in self.current.iter().enumerate() {
            let has_neighbours = self.compute_steering(*boid, &mut steering);
            let mut target: &mut Boid = &mut self.next[i];
            target.position = boid.position;
            target.velocity = boid.velocity;
            target.update_position(dt);

            if has_neighbours {
                let current = target.velocity;

                target.velocity.add_scaled(&steering.separation, self.parameters.separation_factor);
                target.velocity.add_scaled(&steering.alignment, self.parameters.alignment_factor);
                target.velocity.add_scaled(&steering.cohesion, self.parameters.cohesion_factor);
//                target.velocity.add_scaled(&current, -self.parameters.cohesion_factor);
//                target.velocity.add_scaled(&current, -self.parameters.separation_factor);
                target.velocity.add_scaled(&current, -self.parameters.alignment_factor);


                target.velocity.x += target.velocity.x * (2.0 * rng.gen::<f32>() - 1.0) * RANDOM_FACTOR;
                target.velocity.y += target.velocity.y * (2.0 * rng.gen::<f32>() - 1.0) * RANDOM_FACTOR;

                target.update_speed();
                target.clamp_speed(self.parameters.min_bird_speed, self.parameters.max_bird_speed);
            }
            constraint_boid_rect(&mut target, self.playfield_size);
        }

        self.current.swap_with_slice(&mut self.next);
    }


    fn compute_steering(&self, reference: Boid, steering: &mut Steering) -> bool {
        let mut buffer = Vector { x: 0., y: 0. };
        steering.clear();

        let mut nb_neighbour = 0;
        for boid in self.current.iter() {
            let visible = self.compute_separation(reference, *boid, &mut buffer);
            if visible {
                nb_neighbour += 1;
                steering.separation.add(&buffer);
                steering.alignment.add(&boid.velocity);
                steering.cohesion.add(&boid.position);
            }
        }
        if nb_neighbour > 0 {
            steering.alignment.scale(1. / (nb_neighbour as f32));
            steering.cohesion.scale(1. / (nb_neighbour as f32));
            steering.cohesion.subtract(&reference.position);
            return true;
        }
        return false;
    }

    fn compute_separation(&self, reference: Boid, other: Boid, separation: &mut Vector) -> bool {
        *separation = reference.position;
        separation.subtract(&other.position);

        let distance = separation.hypot();

        let prod = (separation.x * reference.velocity.x + separation.y * reference.velocity.y) / (distance * reference.speed);

        let outside_of_visibility = distance > self.parameters.visibility_radius;
        let in_dead_angle = prod < self.parameters.cos_max_angle;

        if outside_of_visibility || in_dead_angle {
            return false;
        }

        true
    }
}



fn constraint_boid_rect(boid: &mut Boid, playfield_size: f32) {
    let var = CONSTRAINT_STRENGTH;
    let limit = playfield_size * 0.8;

    if boid.position.x > limit {
        boid.velocity.x -= var * (boid.position.x - limit);
    }
    if boid.position.x < -limit {
        boid.velocity.x += var * (-limit - boid.position.x);
    }
    if boid.position.y > limit {
        boid.velocity.y -= var * (boid.position.y - limit);
    }
    if boid.position.y < -limit {
        boid.velocity.y += var * (-limit - boid.position.y);
    }
}

fn constraint_boid_tanh(boid: &mut Boid, playfield_size: f32) {
    let distance = boid.position.hypot();
    let limit = playfield_size;

    let d = (1.0 + ((distance - limit) * 100.0 / limit).tanh()) * CONSTRAINT_STRENGTH;

    boid.velocity.x -= boid.position.x / distance * d;
    boid.velocity.y -= boid.position.y / distance * d;
}
