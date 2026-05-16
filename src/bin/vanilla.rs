// vanilla.rs

fn fn_eq(a: fn(&mut F, &E) -> T, b: fn(&mut F, &E) -> T) -> bool {
    std::ptr::fn_addr_eq(a, b)
}

#[derive(Debug)]
enum E {
    Up,
    Down,
}

struct F {
    k: i32,
    t: T,
}

struct T(fn(&mut F, &E) -> T);

impl F {
    fn new() -> Self {
        Self {
            k: 0,
            t: T(Self::low),
        }
    }

    fn low(&mut self, e: &E) -> T {
        println!("     low: {:?}", e);
        match e {
            E::Up => T(Self::high),
            E::Down => T(Self::low),
        }
    }

    fn high(&mut self, e: &E) -> T {
        println!("    high: (k: {}) {:?}", self.k, e);
        const MAX_K: i32 = 2;
        match e {
            E::Up => {
                self.k += 1;
                if self.k > MAX_K {
                    T(Self::terminal)
                } else {
                    T(Self::high)
                }
            }
            E::Down => {
                self.k = 0;
                T(Self::low)
            }
        }
    }

    fn terminal(&mut self, e: &E) -> T {
        println!("terminal: {:?}", e);
        T(Self::terminal)
    }

    fn is_terminal(&self) -> bool {
        let T(phi) = self.t;
        fn_eq(phi, Self::terminal)
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

use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
    let mut a = F::new();

    let seed: u64 = 704033;
    let mut omega = Xoshiro256PlusPlus::seed_from_u64(seed);

    for _ in 1..40 {
        if !a.is_terminal() {
            let u = omega.gen::<E>();
            a.handle(&u);
        }
    }
    a.handle(&E::Up);

    println!("ok");
}

// end
