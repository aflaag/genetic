use crate::ga::member::Member;
use crate::ga::geneticiter::GeneticIter;

use std::marker::PhantomData;

use rand::Rng;

pub struct Genetic<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    target: M,
    size: usize,
    mutation_rate: f32,
    rng: R,
    _marker: PhantomData<M>,
}

impl<M, R> Genetic<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    pub fn new(target: M, size: usize, mutation_rate: f32, rng: R) -> Self {
        Self {
            target,
            size,
            mutation_rate,
            rng,
            _marker: PhantomData,
        }
    }
}

impl<M, R> IntoIterator for Genetic<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    type Item = M;

    type IntoIter = GeneticIter<M, R>;

    fn into_iter(self) -> Self::IntoIter {
        GeneticIter::new(self.target, self.size, self.mutation_rate, self.rng)
    }
}
