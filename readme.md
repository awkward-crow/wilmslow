# wilmslow -- messing with finite state machines

## latest

 - random events with user supplied rng
 - random events
 - events Up, Down

## next steps

 - functions to model states low, high
 - add terminal state
 - compare states i.e. functions for equality
 - state_name : states -> &str
 - optional internal events e.g. scheduler with restarts

and at some point some logging

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

## `cargo add ...`

```sh
cargo add rand
cargo add rand_xoshiro
```

### end
