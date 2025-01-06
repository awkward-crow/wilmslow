# wilmslow -- messing with finite state machines

## latest

 - state: states -> &str
 - add terminal state and is_terminal() 
 - count for time spent in state high and with events generated at random
 - functions to model states low, high
 - random events with user supplied rng
 - random events
 - events Up, Down

## next steps

 - optional internal events e.g. scheduler with restarts

and at some point some logging and **testing**

## functions as states

see https://github.com/nlfiedler/shiranui/blob/master/src/lexer.rs for use of struct containing a function for states

## random events

Using `rand::random`

```rust
let u = rand::random::<E>();
println!("{:?}", u);

for _ in 1..10 {
    let v: E = rand::random();
    println!("{:?}", v);
}
```

Or, with a custom rng,

```rust
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

let seed: u64 = 4033;
let mut omega = Xoshiro256PlusPlus::seed_from_u64(seed);

for _ in 1..10 {
    let u = omega.gen::<E>();
    println!("{:?}", u);
}
```

## `cargo add ...`

```sh
cargo add rand
cargo add rand_xoshiro
```

## terminal state

```rust
let mut a = F { k: 0, t: T(F::low) };

for e in [E::Up, E::Up, E::Up, E::Up, E::Down, E::Down, E::Up] {
    a.handle(&e);
}
```
 

### end
