use crate::*;

pub trait Individual {
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
    fn create(chromosome: Chromosome) -> Self;
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq)]
pub enum TestIndividual {
    // for tests that require access to chromosome
    WithChromosome { chromosome: Chromosome },

    // for tests that don't require access to chromosome
    WithFitness { fitness: f32 },
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }
    
    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,

            Self::WithFitness { .. } => {
                panic!("not supported for TestIndividual::WithFitness");
            }
        }
        
    }
    
    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => {
                chromosome.iter().sum()
                // the simplest fitness test ever, we just sum all the genes together
            }

            Self::WithFitness { fitness } => *fitness,
        }
    }

}