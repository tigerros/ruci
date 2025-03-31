[![build](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/correctness.yml?label=build)](https://github.com/tigerros/ruci/actions/workflows/correctness.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)
[![license](https://img.shields.io/crates/l/ruci)](https://github.com/tigerros/ruci/blob/master/LICENSE)

# ruci
**R**ust **U**niversal **C**hess **I**nterface.

This crate parses and creates UCI messages.
It follows the [UCI protocol](https://backscattering.de/chess/uci) and uses [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.
The UCI protocol is the most widely used way for GUI's to communicate with engines and vice versa.

`#![no_std]` compatible.

See the [examples](https://github.com/tigerros/ruci) for a demo on how to send and receive messages.
You can run each one with `cargo run --package <example-name>`.

## Comparison
There's two other crates that I'm aware of which serve a similar purpose. *Keep in mind that this is a shallow comparison, I haven't looked extensively and I am not an expert.*

- [`vampirc-uci`](https://crates.io/crates/vampirc-uci):
  - Doesn't use `shakmaty`, which AFAIK is the go-to chess crate now.
  - Doesn't separate the two types of messages (engine, GUI) and specific messages. It just has one big enum which mostly uses enum fields for message data. This is really inconvenient because you can't represent specific messages, only the whole `Message` enum.
  - ~~Doesn't provide IO communication with an engine.~~ There is [`vampirc-io`](https://crates.io/crates/vampirc-io), but the API is lacking and it uses the deprecated [`async-std`](https://crates.io/crates/async-std) crate.
  - More dependencies; [`pest`](https://crates.io/crates/pest) and [`chrono`](https://crates.io/crates/chrono). `ruci` only has shakmaty and two macros, which don't get included in the final binary.
  - Not `#![no_std]` compatible.
  - More tests, but I don't know about the coverage.
- [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci): this library is based on/inspired by `vampirc-uci`, so all of the above bullet points apply, except:
  - Uses `shakmaty`.
  - Uses [`nom`](https://crates.io/crates/nom) instead of `pest` and doesn't use `chrono`.
  - Is `#![no_std]` compatible.

*`ruci` might also faster since it doesn't use a parsing library, but I'm not making any claims or showing results because I only have some
toy benchmarks (but yes, they do technically favor `ruci`).*

## Feature flags
- `default`: no features are enabled by default.
- `engine-sync`: adds the `Engine` struct for communicating with an engine. Requires `std`.
- `engine-async`: adds the `EngineAsync` struct for communicating with an engine using [`tokio`](https://crates.io/crates/tokio). Also enables the `engine-sync` feature because of shared error types.
- `serde`: enables serde support for most types. All implementations are derived with no parameters. Requires `std`.