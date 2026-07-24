# learning-rust

A learning path through the Rust programming language, from absolute
zero to the basics of concurrency and async. Sixteen independent
examples, each runnable and tested on its own.

French version: [README.md](README.md)

## Why this repository

Not a tutorial to read, a set of short programs to run, tweak, and
deliberately break to see what the Rust compiler has to say about it.
Each file in `examples/` covers one concept, in the order it makes
sense to discover them.

## Prerequisites

- [Rust](https://www.rust-lang.org/) installed via [rustup](https://rustup.rs/):

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- No other dependency: `cargo` (bundled with Rust) is all you need to
  build, run, and test everything.

## Running an example

Every file in `examples/` is a standalone mini-program.

```bash
cargo run --example 01_hello_world
cargo run --example 08_enums_and_pattern_matching
```

## Running the tests

Each example ships its own unit tests (`#[cfg(test)]`). One command
runs the whole repository:

```bash
cargo test
```

## Checking the style (optional)

This repository is clean under [clippy](https://github.com/rust-lang/rust-clippy),
Rust's official linter:

```bash
rustup component add clippy
cargo clippy --examples --all-targets
```

A few clippy warnings are deliberately disabled (via a top-of-file
`#![allow(...)]` with a comment explaining why) whenever the
"idiomatic" rewrite would hide the concept being taught.

## Examples overview

| # | File | Concept |
|---|------|---------|
| 01 | [`hello_world`](examples/01_hello_world.rs) | First program, `println!`, comments |
| 02 | [`variables_and_types`](examples/02_variables_and_types.rs) | Variables, mutability, shadowing, scalar types, tuples, arrays |
| 03 | [`functions`](examples/03_functions.rs) | Functions, expressions vs statements, returning `Option` |
| 04 | [`control_flow`](examples/04_control_flow.rs) | `if`/`else`, `loop`, `while`, `for`, loop labels |
| 05 | [`ownership`](examples/05_ownership.rs) | Ownership, moves, `Copy`, `clone`, scope and `drop` |
| 06 | [`borrowing_and_references`](examples/06_borrowing_and_references.rs) | `&`/`&mut` borrows, borrow checker rules, slices |
| 07 | [`structs`](examples/07_structs.rs) | Structs, `impl`, methods, associated functions |
| 08 | [`enums_and_pattern_matching`](examples/08_enums_and_pattern_matching.rs) | Enums, exhaustive `match`, `Option`, `if let` |
| 09 | [`collections`](examples/09_collections.rs) | `Vec`, `String`, `HashMap` |
| 10 | [`error_handling`](examples/10_error_handling.rs) | `Option`, `Result`, the `?` operator, `panic!` |
| 11 | [`generics_and_traits`](examples/11_generics_and_traits.rs) | Generics, traits, default methods, `dyn Trait` |
| 12 | [`lifetimes`](examples/12_lifetimes.rs) | Lifetime annotations, structs holding references, `'static` |
| 13 | [`closures_and_iterators`](examples/13_closures_and_iterators.rs) | Closures, environment capture, iterator adapters |
| 14 | [`smart_pointers`](examples/14_smart_pointers.rs) | `Box`, `Rc`, `RefCell`, interior mutability |
| 15 | [`concurrency`](examples/15_concurrency.rs) | Threads, `Arc<Mutex<T>>`, `mpsc` channels |
| 16 | [`async_intro`](examples/16_async_intro.rs) | `async`/`await`, the Tokio runtime, `join!`, `spawn` |

## Repository layout

A single Cargo crate (`learning-rust`), with no real library code
(`src/lib.rs` is intentionally empty of logic). All the teaching
content lives in `examples/`, where each file is both a runnable
binary and a test module.

```
learning-rust/
  Cargo.toml
  examples/
    01_hello_world.rs
    ...
    16_async_intro.rs
  src/
    lib.rs
```

## Status

All 16 concepts listed above are written, tested, and clippy-clean.
Ideas for later, with no fixed date: multi-file crate organization and
modules, declarative macros, hand-written `Iterator`/`From`/`Into`
implementations, a small wrap-up CLI project combining several of
these concepts.

## License

[MIT](LICENSE) - (c) 2026 Riadh MNASRI
