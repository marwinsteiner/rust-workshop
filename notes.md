intllij was first, 2001
pycharm, 2010

A Rust function definition is using
```{rust}
fn is_even(n: i32) -> bool {
    n % 2 == 0
}
```

We call expected behavior (input, output) a contract.

Rust parses explicitly.

Owner: holds the borrowed value, when the owner goes out of scope, the value is dropped.

Borrower: uses a reference to access the value. The original owner still owns it.

Borrow a reference with "&"

## Our game's contract
- Three explicit moves: rock, paper, scissors
- Invalid input retries: no score change. No training example
- Previous rounds only: the bot chooses before the current input.

Goal: build a rock paper scissors bot to play against a human.

## Cargo, rustc, and RustRover
rustc: the rust compiler - checks code and produces machine code

cargo: package manager + build tool - dependencies, build

RustRover: JetBrains IDE for Rust - editing, error highlighting, running and debugging (uses cargo and rustc under the hood)


