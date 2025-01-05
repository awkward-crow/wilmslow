// main.rs

#[derive(Debug)]
enum E {
    Up,
    Down,
}

// random choice of events E, see https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<E> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> E {
        match rng.gen_range(0..=1) {
            0 => E::Up,
            _ => E::Down,
        }
    }
}

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    println!("{:?}", E::Up);

    let u = rand::random::<E>();
    println!("{:?}", u);

    for _ in 1..10 {
        let v: E = rand::random();
        println!("{:?}", v);
    }

    let seed: u64 = 4033;
    let mut omega = Xoshiro256PlusPlus::seed_from_u64(seed);

    for _ in 1..10 {
        let u = omega.gen::<E>();
        println!("{:?}", u);
    }

    println!("ok");
}

// end
