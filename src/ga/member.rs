use crate::ga::gene::Gene;

use rand::{rngs::ThreadRng, Rng};

pub trait Member: Clone + PartialEq {
    type MemberGene: Gene;

    type FitnessOutput: Ord + std::fmt::Display + std::fmt::Debug;

    type Rng: Rng + ?Sized + Clone;

    fn from_random(rng: &mut Self::Rng) -> Self;

    fn fitness(&self, target: Self) -> Self::FitnessOutput;

    fn breed(&self, other: Self, rng: &mut Self::Rng) -> Self;

    fn mutate(&mut self, mutation_rate: f32, rng: &mut Self::Rng);
}

impl<const S: usize> Member for [u8; S] {
    type MemberGene = u8;

    type FitnessOutput = usize;

    type Rng = ThreadRng;

    fn from_random(rng: &mut Self::Rng) -> Self {
        let mut bytes = [0; S];

        bytes
            .iter_mut()
            .for_each(|byte| *byte = Self::MemberGene::from_random(rng));

        bytes
    }

    fn fitness(&self, target: Self) -> Self::FitnessOutput {
        self
            .iter()
            .zip(target.iter())
            .map(|(self_gene, target_gene)| (*self_gene as isize - *target_gene as isize).abs() as usize)
            .sum()
    }

    fn breed(&self, other: Self, rng: &mut Self::Rng) -> Self {
        let mut child = [0; S];
        
        std::iter::zip(*self, other)
            .zip(child.iter_mut())
            .for_each(|((m, d), c)| *c = if rng.gen() { d } else { m });

        child
    }

    fn mutate(&mut self, mutation_rate: f32, rng: &mut Self::Rng) {
        self
            .iter_mut()
            .for_each(|byte| if rng.gen::<f32>() < mutation_rate { *byte = rng.gen() })
    }
}
