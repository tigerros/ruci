[![build](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/correctness.yml?label=build)](https://github.com/tigerros/ruci/actions/workflows/correctness.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)
[![license](https://img.shields.io/crates/l/ruci)](https://github.com/tigerros/ruci/blob/master/LICENSE)

# RUCI
<ins>R</ins>ust <ins>U</ins>niversal <ins>C</ins>hess <ins>I</ins>nterface.

This crate parses and creates UCI messages.
It follows the [UCI standard](https://backscattering.de/chess/uci) and uses [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

`#![no_std]` compatible, unless you enable the `engine-connection` feature.

See the examples for a demo on how to send and receive messages.

## Comparison
There's two other crates that I'm aware of which serve a similar purpose. *Keep in mind that this is a shallow comparison, I haven't looked extensively and I am not an expert.*

- [`vampirc-uci`](https://crates.io/crates/vampirc-uci):
  - Doesn't use shakmaty, which AFAIK is the go-to chess crate now.
  - API problems:
    - Doesn't separate GUI and engine messages. This is bad if you want to communicate with an engine/GUI, because you're going to need functions like `send_message` and `read_message`, where you want to specify which type of message you are sending and receiving. It's not impossible to do this with `vampirc-uci`, but you won't have strong type guarantees.
    - Doesn't have separate structs/enums for messages. Similar to the above, this is bad if you want to represent a specific message. With `vampirc-uci`, you can only represent the whole enum.
  - More dependencies; `pest` and `chrono`. RUCI only has shakmaty and two macros which don't get included in the final binary.
  - Not `#![no_std]` compatible.
  - More tests than RUCI.
  - Doesn't provide IO communication with an engine.
- [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci):
  - This library is based on/inspired by `vampirc-uci`, so all of the above bullet points apply, except:
    - Uses shakmaty.
    - Uses `nom` for parsing and doesn't have any other dependencies.
    - Is `#![no_std]` compatible.

*RUCI might also faster since it doesn't use a parsing library, but I'm not making any claims or showing results because I only have some
toy benchmarks (but yes, they do technically favor RUCI).*

## Feature flags
- `engine-connection`: enables a structs to manage the IO when it comes to working with a UCI engine. Note that this will add [`tokio`](https://crates.io/crates/tokio) and [`parking_lot`](https://crates.io/crates/parking_lot) as dependencies. If you're making an engine, listen to stdin and parse it into a `gui::Message`, then print a string representation of an `engine::Message`.
- `serde`: enables `Serialize` and `Deserialize` for most types. All of the implementations are derived with no parameters.