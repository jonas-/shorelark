use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Simulation {
    world: World,
}
#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f64>,
    rotation: na::Rotation2<f64>,
    speed: f64,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
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

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}
