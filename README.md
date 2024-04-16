[![docs.rs](https://img.shields.io/docsrs/ruci?logo=docs.rs&label=docs.rs)](https://docs.rs/ruci/)
[![crates.io](https://img.shields.io/crates/v/ruci?logo=rust)](https://crates.io/crates/ruci)
[![license](https://img.shields.io/crates/l/ruci)](https://github.com/tigerros/ruci/blob/master/LICENSE)

# RUCI

**Warning: This crate has no tests and I'm not confident in it. I'm publishing it in order to comfortably use it in another project of mine, I do not guarantee that it works for you.**

<ins>R</ins>ust <ins>U</ins>niversal <ins>C</ins>hess <ins>I</ins>nterface.

This crate is for parsing and creating UCI messages.
It follows the [UCI standard](https://backscattering.de/chess/uci).

## Features

- `uci-connection` (default): Enables two "helper" structs to manage the actual IO (`GuiToEngineUciConnection` and `EngineToGuiUciConnection`) when it comes to working with UCI.