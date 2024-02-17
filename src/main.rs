use genetic::ga::genetic::Genetic;

use rand::rngs::ThreadRng;

fn main() {
    let target = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let rng = rand::thread_rng();

    let genetic: Genetic<_, ThreadRng> = Genetic::new(target, 100, 0.05, rng);

    let result = genetic
        .into_iter()
        .find(|best| *best == target)
        .unwrap();

    println!("{:?}", result);
}
