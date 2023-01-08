#![feature(type_alias_impl_trait)]

use rand::seq::SliceRandom;
use rand::Rng;
use rand::RngCore;

pub use self::{chromosome::*, crossover::*, individual::*, mutation::*, selection::*};

mod chromosome;
mod crossover;
mod individual;
mod mutation;
mod selection;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // selection
                let parent_a = self.selection_method.select(rng, population).chromosome();
                let parent_b = self.selection_method.select(rng, population).chromosome();

                // crossover
                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                // mutation
                self.mutation_method.mutate(rng, &mut child);

                // convert Chromosome back into individual
                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        // check fitness improved
        let len = population.len() as f32;
        let average_fitness_init = population.iter().map(|x| x.fitness()).sum::<f32>() / len;

        println!(
            "Average fitness - initial population: {}",
            average_fitness_init
        );

        // println!("Average fitness - initial population: {}", average_fitness);

        // we run .evolve() a few times, to spot the difference between initial and output population
        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        // check fitness improved
        let average_fitness_evolve = population.iter().map(|x| x.fitness()).sum::<f32>() / len;

        println!(
            "Average fitness - evolved population: {}",
            average_fitness_evolve
        );

        // check if fitness really improved
        assert!(average_fitness_init <= average_fitness_evolve);

        let expected_population = vec![
            individual(&[0.4476949, 2.0648358, 4.3058133]),
            individual(&[1.2126867, 1.5538777, 2.886911]),
            individual(&[1.0617678, 2.265739, 4.428764]),
            individual(&[0.95909685, 2.4618788, 4.024733]),
        ];

        assert_eq!(population, expected_population);
    }
}
