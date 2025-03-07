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
There's two other crates that I'm aware of which serve a similar purpose. *Keep in mind that this is a shallow comparison, I haven't looked extensively and I am not an expert.*

- [`vampirc-uci`](https://crates.io/crates/vampirc-uci):
  - Doesn't use shakmaty, which AFAIK is the go-to chess crate now.
  - Uses [`pest`](https://pest.rs/) for parsing (ruci does it manually).
  - More dependencies; `pest` and `chrono`. Ruci only has shakmaty and two macros which don't get included in the final binary.
  - Lot more tests than ruci. However, ruci has simpler parsing and less code. But yes, I am guilty.
  - Doesn't separate GUI and engine messages. This is bad if you want to communicate with an engine/GUI, because you're going to need functions like `send_message` and `read_message`, where you want to specify which message you are sending and receiving.
  - Less direct implementation of the UCI standard.
  - Maybe a pet peeve, but sometimes uses enum fields instead of separate types. IMHO this is bad because you can't represent just an `id` message, because the fields are baked into the whole enum.
  - Doesn't provide the actual IO communication with an engine.
- [`shakmaty-uci`](https://crates.io/crates/shakmaty-uci):
  - This library is based on/inspired by `vampirc-uci`, so all of the above bullet points apply, except:
    - Uses shakmaty.
    - Uses [`nom`](https://crates.io/crates/nom) for parsing.
    - Doesn't have any other dependencies.

## Feature flags
- `engine-connection`: enables a structs to manage the IO when it comes to working with a UCI engine. Note that this will add [`tokio`](https://crates.io/crates/tokio) and [`parking_lot`](https://crates.io/crates/parking_lot) as dependencies. If you're making an engine, listen to stdin and parse it into a `gui::Message`, then print a string representation of an `engine::Message`.