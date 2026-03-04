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
        match e {
            E::Failed => (Some(E::Start), T(Self::init)),
            E::Complete => (None, T(Self::complete)),
            _ => (None, T(Self::running))
        }
    }

    fn is_init(&self) -> bool {
        let T(phi) = self.t;
        std::ptr::fn_addr_eq(phi, Self::init as for<'a, 'b> fn(&'a mut F, &'b E) -> (Option<E>, T))
    }

    fn is_running(&self) -> bool {
        let T(phi) = self.t;
        std::ptr::fn_addr_eq(phi, Self::running as for<'a, 'b> fn(&'a mut F, &'b E) -> (Option<E>, T))
    }

    fn complete(&mut self, _e: &E) -> (Option<E>, T) {
(None, T(Self::complete))
    }

    fn is_complete(&self) -> bool {
        let T(phi) = self.t;
        std::ptr::fn_addr_eq(phi, Self::complete as for<'a, 'b> fn(&'a mut F, &'b E) -> (Option<E>, T))
    }

    fn failed(&mut self, _e: &E) -> (Option<E>, T) {
(None, T(Self::failed))
    }

    fn is_failed(&self) -> bool {
        let T(phi) = self.t;
        std::ptr::fn_addr_eq(phi, Self::failed as for<'a, 'b> fn(&'a mut F, &'b E) -> (Option<E>, T))
    }

    fn state(&self) -> &str {
        if self.is_running() {
            "running"
        } else if self.is_complete() {
            "complete"
        } else if self.is_failed() {
            "failed"
        } else if self.is_init() {
            "init"
        } else {
            "unknown"
        }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts() {
        let mut m = F { k: 0, t: T(F::init) };
        assert_eq!(m.k, 0);
        m.handle(&E::Start);
        assert_eq!(m.k, 1)
    }

    #[test]
    fn reach_running() {
        let mut m = F { k: 0, t: T(F::init) };
        m.handle(&E::Start);
        m.handle(&E::Running);
        assert!(m.is_running())
    }

    #[test]
    fn reach_complete() {
        let mut m = F { k: 0, t: T(F::init) };
        m.handle(&E::Start);
        m.handle(&E::Running);
        m.handle(&E::Complete);
        assert!(m.is_complete())
    }

    #[test]
    fn restart() {
        let mut m = F { k: 0, t: T(F::init) };
        m.handle(&E::Start);
        m.handle(&E::Running);
        m.handle(&E::Failed);
        assert_eq!(m.k, 2)
    }

    #[test]
    fn reach_failed() {
        let mut m = F { k: 0, t: T(F::init) };
        m.handle(&E::Start);
        for _ in 1..=MAX_STARTS {
            m.handle(&E::Running);
            m.handle(&E::Failed);
        }
        assert!(m.is_failed())
    }

}


fn main() {
    let mut m = F { k: 0, t: T(F::init) };

    for line in std::io::stdin().lines() {
        let s = line.unwrap();
        let e = E::from_str(&s);
        if let Ok(v) = e {
            print!("{}({}) [{:?}]", m.state(), m.k, v);
            m.handle(&v);
            println!(" -> {}({})", m.state(), m.k);
        } else {
            println!("{} - not recognised", s);
        }

    }

    println!("done");
}

// end
