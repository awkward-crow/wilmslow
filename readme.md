# wilmslow -- messing with finite state machines

## latest

 - count for time spent in state high and with events generated at random
 - functions to model states low, high
 - random events with user supplied rng
 - random events
 - events Up, Down

## next steps

 - add terminal state 
 - compare states i.e. functions for equality
 - state_name : states -> &str
 - optional internal events e.g. scheduler with restarts

and at some point some logging

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

### end
