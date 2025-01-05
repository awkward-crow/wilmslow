// main.rs


#[derive(Debug)]
enum E {
    Up,
    Down,
}


struct F {
    t: T
}


struct T (fn(&mut F, &E) -> T);

impl F {
    fn low(&mut self, e: &E) -> T {
        println!("low: {:?}", e);
        match e {
            E::Up => T(Self::high),
            E::Down => T(Self::low)
        }
    }

    fn high(&mut self, e: &E) -> T {
        println!("high: {:?}", e);
        match e {
            E::Up => T(Self::high),
            E::Down => T(Self::low)
        }
    }

    fn handle(&mut self, e: &E) -> bool {
        let T(phi) = self.t;
        self.t = phi(self, e);
        true
    }
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

fn main() {
    let mut a = F { t: T(F::low) };

    for e in [E::Down, E::Up, E::Up, E::Down, E::Down] {
        a.handle(&e);
    }

    println!("ok");
}

// end
