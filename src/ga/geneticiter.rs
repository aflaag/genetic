use crate::ga::member::Member;

use rand::Rng;
use rand::seq::SliceRandom;

pub struct GeneticIter<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    target: M,
    size: usize,
    half: usize,
    mutation_rate: f32,
    population: Vec<M>,
    rng: R,
}

impl<M, R> GeneticIter<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    pub fn new(target: M, size: usize, mutation_rate: f32, mut rng: R) -> Self {
        let mut population: Vec<M> = (0..size).map(|_| M::from_random(&mut rng)).collect();

        population.sort_by_cached_key(|member| member.fitness(target.clone()));

        Self {
            target,
            size,
            half: size / 2,
            mutation_rate,
            rng,
            population,
        }
    }

    fn repopulate(&mut self) {
        (0..self.size - self.half)
            .for_each(|_| {
                let mother = self.population.choose(&mut self.rng).unwrap();
                let father = self.population.choose(&mut self.rng).unwrap();

                let mut children = mother.breed(father.clone(), &mut self.rng);

                children.mutate(self.mutation_rate, &mut self.rng);

                self.population.push(children);
            });
    }
}

impl<M, R> Iterator for GeneticIter<M, R>
where
    M: Member<Rng = R>,
    R: Rng + ?Sized + Clone,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        // remove the last half of the population
        self.population.drain(self.half..);

        // repopulate the population with new members
        self.repopulate();

        // sort the population by the fitness
        self.population.sort_by_key(|member| member.fitness(self.target.clone()));

        // the best memeber of the population is going to be the first member
        Some(self.population[0].clone())
    }
}
