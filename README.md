[![build](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/correctness.yml?label=build)](https://github.com/tigerros/ruci/actions/workflows/correctness.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)
[![license](https://img.shields.io/crates/l/ruci)](https://github.com/tigerros/ruci/blob/master/LICENSE)

# RUCI
<ins>R</ins>ust <ins>U</ins>niversal <ins>C</ins>hess <ins>I</ins>nterface.

This crate parses and creates UCI messages.
It follows the [UCI standard](https://backscattering.de/chess/uci) and uses [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

See the examples for a demo on how to send and receive messages.

## Comparison
There's two other crates that I'm aware of which serve a similar purpose. Here's a small comparison but keep in mind I haven't looked extensively.

- [`vampirc-uci`](https://crates.io/crates/vampirc-uci):
  - Doesn't use shakmaty, which AFAIK is the go-to chess crate now.
  - Uses [`pest`](https://pest.rs/) for parsing (ruci does it manually).
  - More dependencies; `pest` and `chrono`. Ruci only has shakmaty and two macros which don't get included in the final binary.
  - Lot more tests than ruci. However, ruci has simpler parsing and less code. But yes, I am guilty.
  - Doesn't separate GUI and engine-bound messages.
  - Less direct implementation of the UCI standard.
  - Maybe a pet peeve, but sometimes uses enum fields instead of separate types. IMHO this is bad because you can't represent just an `id` message, because the fields are baked into the whole enum.
- [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci):
  - Very similar to vampirc, but uses shakmaty.
  - Uses [`nom`](https://crates.io/crates/nom) for parsing.

Neither of these have the means to connecting to an engine.

## Feature flags
- `engine-connection`: enables a structs to manage the IO when it comes to working with a UCI engine. Note that this will add [`tokio`](https://crates.io/crates/tokio) and [`parking_lot`](https://crates.io/crates/parking_lot) as dependencies. If you're making an engine, listen to stdin and parse it into a `gui::Message`, then print a string representation of an `engine::Message`.