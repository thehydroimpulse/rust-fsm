Finite State Machine (FSM) for Rust
---

Rust-fsm is finite state machine implementation in Rust. It exposes specific macros to make it fairly expressive to work with. Currently, Scala-like syntax is an inspiration for this library. You can get pretty far with macros, except Rust is currently limited in mutli-statement macros.


## Install

Todo

## Usage

```rust
extern mod fsm;

use fsm;

fn main() {
    // Create a new set of states.
    // `State` defines a new module where each state is defined in.
    defstates! (State -> Unlocked, Locked);

    // Create a new FSM and pass an initial state:
    let mut machine = fsm::StateMachine::new(State::Unlocked);

    // Do something when we lock the machine:
    machine.when(State::Locked, || {
        println!("We have locked it again.");
    });

    machine.switch(State::Locked);
}
```

## Docs

```
make docs
open docs/fsm/index.html
```

## License

MIT