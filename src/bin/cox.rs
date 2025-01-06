// cox.rs

#[derive(Debug)]
enum E {
    Start,
    Running,
    Complete,
    Failed,
}

use std::str::FromStr;

impl FromStr for E {
    type Err = ();

    fn from_str(input: &str) -> Result<E, Self::Err> {
        match input {
            "start" => Ok(E::Start),
            "running" => Ok(E::Running),
            "complete" => Ok(E::Complete),
            "failed" => Ok(E::Failed),
            _ => Err(())
        }
    }
}

const MAX_STARTS: i32 = 2;

struct F {
    k: i32,
    t: T
}

struct T (fn(&mut F, &E) -> (Option<E>, T));

impl F {
    fn init(&mut self, e: &E) -> (Option<E>, T) {
        println!("init: {:?}", e);
        match e {
            E::Start => {
                self.k += 1;
                if self.k > MAX_STARTS {
                    (None, T(Self::failed))
                } else {
                    (None, T(Self::init))
                }
            },
            E::Running => {
                (None, T(Self::running))
            },
            _ => (None, T(Self::init))
        }
    }

    fn running(&mut self, e: &E) -> (Option<E>, T) {
        println!("running: {:?}", e);
        match e {
            E::Failed => (Some(E::Start), T(Self::init)),
            E::Complete => (None, T(Self::complete)),
            _ => (None, T(Self::running))
        }
    }

    fn complete(&mut self, e: &E) -> (Option<E>, T) {
        println!("complete: {:?}", e);
        (None, T(Self::complete))
    }

    fn failed(&mut self, e: &E) -> (Option<E>, T) {
        println!("failed: {:?}", e);
        (None, T(Self::failed))
    }

    fn handle(&mut self, e: &E) -> bool {
        let T(phi) = self.t;
        let (v, s) = phi(self, e);
        self.t = s;
        if let Some(u) = v {
            self.handle(&u)
        } else {
            true
        }
    }
}


fn main() {
    let mut m = F { k: 0, t: T(F::init) };

    for line in std::io::stdin().lines() {
        let e = E::from_str(&line.unwrap());
        if let Ok(v) = e {
            m.handle(&v);
        }
    }

    println!("done");
}

// end
