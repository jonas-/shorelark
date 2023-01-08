use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "Test".into()
}

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();

        Self { animals }
    }
}

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}

// ^ This model is smaller than `lib_simulation::Animal` - that's
// | because a bird's position is all we need on the JavaScript's
// | side at the moment; there's no need to map rest of the fields.
