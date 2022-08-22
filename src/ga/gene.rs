use rand::{Rng, rngs::ThreadRng};

pub trait Gene {
    type Rng: Rng + ?Sized + Clone;

    fn from_random(rng: &mut Self::Rng) -> Self;
}

impl Gene for u8 {
    type Rng = ThreadRng;

    fn from_random(rng: &mut Self::Rng) -> Self {
        rng.gen()
    }
}
