use rand::Rng;

use crate::data::boid::Boid;
use crate::data::steering::Steering;
use crate::data::vector::Vector;

const CONSTRAINT_STRENGTH: f32 = 0.1;

const SAFE_SPACE_RATIO: f32 = 0.8;
const DEFAULT_VISIBILITY_FACTOR: f32 = 3.0;

// in degree
const DEAD_ANGLE: f32 = 20.0;
const DEFAULT_SEPARATION_FACTOR: f32 = 6.0;
const DEFAULT_COHESION_FACTOR: f32 = 4.0;
const DEFAULT_ALIGNMENT_FACTOR: f32 = 10.;

const RANDOM_FACTOR: f32 = 0.0;
const DEFAULT_BIRD_SIZE: f32 = 0.2;
const DEFAULT_BIRD_MIN_SPEED: f32 = 5.0;
const DEFAULT_BIRD_MAX_SPEED: f32 = 16.0;

const NOT_VISIBLE: u8 = 0;
const VISIBLE: u8 = 1;
const IN_SAFE_SPACE: u8 = 2;

pub struct Parameters {
    pub bird_size: f32,
    pub min_bird_speed: f32,
    pub max_bird_speed: f32,
    pub visibility_radius: f32,
    pub safe_space_ratio: f32,
    dead_angle: f32,
    cos_max_angle: f32,
    pub separation_factor: f32,
    pub cohesion_factor: f32,
    pub alignment_factor: f32,
}


pub fn compute_cos_max_angle(dead_angle: f32) -> f32 {
    (std::f32::consts::PI * (1. - dead_angle / 180.)).cos()
}

impl Parameters {
    pub fn new() -> Self {
        Parameters {
            bird_size: DEFAULT_BIRD_SIZE,
            visibility_radius: DEFAULT_BIRD_SIZE * DEFAULT_VISIBILITY_FACTOR,
            safe_space_ratio: SAFE_SPACE_RATIO,
            dead_angle: DEAD_ANGLE,
            cos_max_angle: compute_cos_max_angle(DEAD_ANGLE),
            cohesion_factor: DEFAULT_COHESION_FACTOR * 0.01,
            separation_factor: DEFAULT_SEPARATION_FACTOR * 0.01,
            alignment_factor: DEFAULT_ALIGNMENT_FACTOR * 0.01,
            min_bird_speed: DEFAULT_BIRD_MIN_SPEED,
            max_bird_speed: DEFAULT_BIRD_MAX_SPEED,
        }
    }

    pub fn dead_angle(&self) -> f32 {
        self.dead_angle
    }

    pub fn cos_max_angle(&self) -> f32 {
        self.cos_max_angle
    }

    pub fn set_dead_angle(&mut self, dead_angle: f32) {
        self.dead_angle = dead_angle;
        self.cos_max_angle = compute_cos_max_angle(dead_angle);
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
            boid.update_speed();

            boid.clamp_speed(
                self.parameters.min_bird_speed,
                self.parameters.max_bird_speed,
            );
        }
    }

    pub fn compute(&mut self, dt: f32) {
        let mut steering = Steering::new();
        let mut rng = rand::thread_rng();

        let nb_birds = self.current.len();
        for i in 0..nb_birds {
            let boid = &self.current[i];
            let has_neighbours = self.compute_steering(*boid, &mut steering);
            let mut target: &mut Boid = &mut self.next[i];
            target.position = boid.position;
            target.velocity = boid.velocity;
            target.update_position(dt);

            if has_neighbours {
                let current = target.velocity;

                target
                    .velocity
                    .add_scaled(&steering.separation, self.parameters.separation_factor);
                target
                    .velocity
                    .add_scaled(&steering.alignment, self.parameters.alignment_factor);
                target
                    .velocity
                    .add_scaled(&steering.cohesion, self.parameters.cohesion_factor);
                target
                    .velocity
                    .add_scaled(&current, -self.parameters.alignment_factor);

                target.velocity.x +=
                    target.velocity.x * (2.0 * rng.gen::<f32>() - 1.0) * RANDOM_FACTOR;
                target.velocity.y +=
                    target.velocity.y * (2.0 * rng.gen::<f32>() - 1.0) * RANDOM_FACTOR;

            }
            target.update_speed();
            target.clamp_speed(
                self.parameters.min_bird_speed,
                self.parameters.max_bird_speed,
            );
            constraint_boid_rect(&mut target, self.playfield_size);
        }

        self.current.swap_with_slice(&mut self.next);
    }

    fn compute_steering(
        &self,
        reference: Boid,
        steering: &mut Steering,
    ) -> bool {
        let mut buffer = Vector { x: 0., y: 0. };
        steering.clear();

        let mut nb_visible = 0;
        let mut nb_in_safe_space = 0;
        for boid in self.current.iter() {
            let visibility = self.compute_separation(reference, *boid, &mut buffer);
            if (visibility & IN_SAFE_SPACE) != 0 {
                let norm2 = buffer.norm();
                if norm2<DEFAULT_BIRD_SIZE*0.1 {
                    buffer.set_random(DEFAULT_BIRD_MIN_SPEED)
                }
                nb_in_safe_space += 1;
                steering.separation.add(&buffer);
            }
            if (visibility & VISIBLE) != 0 {
                nb_visible += 1;
                steering.alignment.add(&boid.velocity);
                steering.cohesion.add(&boid.position);
            }
        }
        //remove myself
        nb_visible -= 1;
        nb_in_safe_space -= 1;
        steering.alignment.subtract(&reference.velocity);
        steering.cohesion.subtract(&reference.position);


        if nb_visible > 0 {
            steering.alignment.scale(1. / (nb_visible as f32));
            steering.cohesion.scale(1. / (nb_visible as f32));
            steering.cohesion.subtract(&reference.position);
            return true;
        }
        return nb_in_safe_space > 0;
    }

    fn compute_separation(&self, reference: Boid, other: Boid, separation: &mut Vector) -> u8 {
        let visibility_radius = self.parameters.visibility_radius;
        *separation = reference.position;
        separation.subtract(&other.position);

        if separation.x.abs() > visibility_radius || separation.y.abs() > visibility_radius {
            return NOT_VISIBLE;
        }

        let distance = separation.hypot();
        if distance > visibility_radius {
            return NOT_VISIBLE;
        }
        let prod = (separation.x * reference.velocity.x + separation.y * reference.velocity.y)
            / (distance * reference.speed());
        if prod < self.parameters.cos_max_angle {
            return NOT_VISIBLE;
        }

        if distance < visibility_radius * self.parameters.safe_space_ratio {
            return IN_SAFE_SPACE | VISIBLE;
        }

        VISIBLE
    }
}

fn constraint_boid_rect(boid: &mut Boid, playfield_size: f32) {
    let var = CONSTRAINT_STRENGTH;
    let limitx = playfield_size * 0.9;
    let limity = playfield_size * 0.8;

    if boid.position.x > limitx {
        boid.velocity.x -= var * (boid.position.x - limitx);
        boid.velocity.y += 0.1;
    }
    if boid.position.x < -limitx {
        boid.velocity.x += var * (-limitx - boid.position.x);
        boid.velocity.y -= 0.1;
    }
    if boid.position.y > limity {
        boid.velocity.y -= var * (boid.position.y - limity);
        boid.velocity.x += 0.1;
    }
    if boid.position.y < -limity {
        boid.velocity.y += var * (-limity - boid.position.y);
        boid.velocity.x -= 0.1;
    }
}
