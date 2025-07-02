[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/test.yml?label=tests)](https://github.com/tigerros/ruci/actions/workflows/test.yml)
[![clippy](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/clippy.yml?label=clippy)](https://github.com/tigerros/ruci/actions/workflows/clippy.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)

# ruci
**R**ust **U**niversal **C**hess **I**nterface.

This crate is a full implementation of the [UCI protocol](https://backscattering.de/chess/uci) using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.
The UCI protocol is the most widely used way to communicate with chess engines and vice versa.

Specifically, this crate contains:
- Types to represent every UCI message.
- The means to convert strings to these types.
- The means to convert these types to a string.
- I/O management for both GUI's and engines (check out the [examples](https://github.com/tigerros/ruci/tree/master/examples), run them with `cargo run -p {example_name}`).

`#![no_std]` compatible.

## Features
No features are enabled by default.

- `engine-sync`: adds the `Engine` struct for communicating with an engine.
- `engine-async`: adds async versions of `Engine` functions.
- `tokio-process`: adds a tokio version of `Engine::from_process`.
- `gui-sync`: adds the `Gui` struct for communicating with a GUI.
- `gui-async`: adds async versions of `Gui` functions.
- `serde`: enables serde support for most types. All implementations are derived with no parameters.

## Comparison
There's two other crates that I'm aware of which serve a similar purpose; [`vampirc-uci`](https://crates.io/crates/vampirc-uci) and [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci).
`shakmaty-uci` is basically an improved version of `vampirc-uci`, so I'll only cover `shakmaty-uci`. Anyways, it:

- Doesn't separate messages. Instead, it has one enum which (mostly) uses enum fields for message data. This is inconvenient because you can't represent specific messages.
- Doesn't provide I/O management.
- Is 3 or more times slower than `ruci` when deserializing, 2 or more times slower when serializing. Results available at [tigerros.github.io/ruci/bench](https://tigerros.github.io/ruci/bench), more compact version in the summary of the latest [Bench workflow](https://github.com/tigerros/ruci/actions/workflows/bench.yml).
- Has more tests, but I don't know about the coverage.
- Uses owned versions of non-copy data. This makes for a cleaner API compared to using `Cow`s (which is what `ruci` does), but it hurts performance. Converting a message to a string doesn't require owned data, and seeing as that is half of the functionality of this crate, it would be wasteful to force ownership.
- Uses [`nom`](https://crates.io/crates/nom) for parsing, whereas `ruci` doesn't pull in anything.

## Benches
`ruci` has more benches per scenario than the other two. This is because it uses `Cow`s to allow for borrowed or owned data. So, the suffix `borrowed` in bench names means that it is using a statically borrowed type rather than the owned type, e.g. `&'static str` instead of `String`. The flip side is the suffix `owned`.

## Safety
`ruci` declares `#![forbid(unsafe_code)]`.