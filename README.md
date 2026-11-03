# Number Guessing Game in Rust

A small command-line game built to learn Rust and practise test-driven development. Guess a randomly chosen number from 0 to 10 in three attempts; the game gives high or low hints. Invalid inputs do not use an attempt.

## Run

Install the Rust toolchain, then run:

```bash
cargo run
```

## Test

```bash
cargo test
```

The tests cover guess comparisons, input validation, attempt counting and the generated number range.
