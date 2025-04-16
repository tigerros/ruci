[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/test.yml?label=tests)](https://github.com/tigerros/ruci/actions/workflows/test.yml)
[![clippy](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/clippy.yml?label=clippy)](https://github.com/tigerros/ruci/actions/workflows/clippy.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)

# ruci
**R**ust **U**niversal **C**hess **I**nterface.

This crate parses and creates UCI messages.
It follows the [UCI protocol](https://backscattering.de/chess/uci) and uses [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.
The UCI protocol is the most widely used way for GUI's to communicate with engines and vice versa.

`#![no_std]` compatible.

See the [examples](https://github.com/tigerros/ruci/tree/master/examples) for demos on how to send and receive messages.
You can run each one with `cargo run --package <example-name>`.

## Comparison
There's two other crates that I'm aware of which serve a similar purpose; [`vampirc-uci`](https://crates.io/crates/vampirc-uci) and [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci).
`shakmaty-uci` is basically an improved version of `vampirc-uci`, so I'll only cover `shakmaty-uci`.

- Doesn't separate the two types of messages (engine, GUI) and specific messages. It has one big enum which mostly uses enum fields for message data. This is inconvenient because you can't represent specific messages, just the whole `UciMessage` enum.
- Doesn't provide IO communication with an engine.
- Uses [`nom`](https://crates.io/crates/nom) for parsing, whereas `ruci` doesn't pull in anything.
- More tests, but I don't know about the coverage.
- 3 or more times slower than `ruci` when deserializing, 2 or more times slower when serializing. Benches don't cover that many types but there's a trend. Results available at [tigerros.github.io/ruci/bench](https://tigerros.github.io/ruci/bench), or you can view a more compact version in the summary of the latest [Bench workflow](https://github.com/tigerros/ruci/actions/workflows/bench.yml) run. Sources at [benches](https://github.com/tigerros/ruci/tree/master/benches).
- `ruci` uses `Cow` for non-copy types. This is because converting a message to a string (which is what half of the library is for) doesn't require owned data, and I got the ick when I had to clone data just to do something that only needs a reference.

## Benches
`ruci` has more benches per scenario than the other two. This is because it uses `Cow`s to allow for borrowed or owned data. So, the suffix `borrowed` in bench names means that it is using a statically borrowed type rather than the owned type, e.g. `&'static str` instead of `String`. The flip side is the suffix `owned`.

## Feature flags
- `default`: no features are enabled by default.
- `engine-sync`: adds the `Engine` struct for communicating with an engine. Requires `std`.
- `engine-async`: adds the `EngineAsync` struct for communicating with an engine using [`tokio`](https://crates.io/crates/tokio). Also enables the `engine-sync` feature because of shared error types.
- `serde`: enables serde support for most types. All implementations are derived with no parameters. Requires `std`.