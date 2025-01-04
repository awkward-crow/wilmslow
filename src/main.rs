// main.rs

// random choice of events E, see https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
enum E {
    Up,
    Down,
}

impl Distribution<E> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> E {
        match rng.gen_range(0..=1) {
            0 => E::Up,
            _ => E::Down,
        }
    }
}

fn main() {
    println!("{:?}", E::Up);
    
    for _ in (1..10) {
        let mut v: E = rand::random();
        println!("{:?}", &v);
    }

    println!("ok");
}


// end
