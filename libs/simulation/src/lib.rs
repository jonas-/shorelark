use nalgebra as na;
use rand::prelude::*;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}
#[derive(Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    pub position: na::Point2<f64>,
    pub rotation: na::Rotation2<f64>,
    pub speed: f64,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f64>,
}

impl Simulation {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Simulation::random(&mut rng)
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    pub fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

        // ^ Our algorithm allows for animals and foods to overlap, so
        // | it's hardly ideal - but good enough for our purposes.
        // |
        // | A more complex solution could be based off of e.g.
        // | Poisson disk sampling:
        // |
        // | https://en.wikipedia.org/wiki/Supersampling
        // ---

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            // ------ ^-------^
            // | If not for `rand-no-std`, we'd have to do awkward
            // | `na::Point2::new(rng.gen(), rng.gen())` instead
            // ---
            rotation: rng.gen(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> na::Point2<f64> {
        // ------------------ ^
        // | No need to return a reference, because na::Point2 is Copy.
        // |
        // | (meaning: it's so small that cloning it is cheaper than
        // | messing with references.)
        // |
        // | Of course you don't have to memorize which types are Copy
        // | and which aren't - if you accidentally return a reference
        // | to a type that's Copy, rust-clippy will point it out and
        // | suggest a change :-)
        // ---

        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f64> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f64> {
        self.position
    }
}
