[![tests](https://img.shields.io/github/actions/workflow/status/tigerros/ruci/tests.yml?label=tests)](https://github.com/tigerros/ruci/actions/workflows/tests.yml)
[![coverage](https://img.shields.io/codecov/c/gh/tigerros/ruci)](https://app.codecov.io/gh/tigerros/ruci/)
[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)
[![license](https://img.shields.io/crates/l/ruci)](https://github.com/tigerros/ruci/blob/master/LICENSE)

# RUCI

<ins>R</ins>ust <ins>U</ins>niversal <ins>C</ins>hess <ins>I</ins>nterface.

This crate is for parsing and creating UCI messages.
It follows the [UCI standard](https://backscattering.de/chess/uci).

See the [go_stop](https://github.com/tigerros/ruci/tree/master/examples/go_stop.rs) example for a demo on how to send and receive messages.

## Features

- `uci-connection`: enables two structs to manage the actual IO (`EngineConnection` and `GuiConnection`) when it comes to working with UCI. Note that this will add [`tokio`](https://crates.io/crates/tokio) and [`parking_lot`](https://crates.io/crates/parking_lot) as dependencies.