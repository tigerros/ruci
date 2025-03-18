//! I don't know what to put here.
//! It's just here to verify that the crate is `#![no_std]`.
//! I do use the attribute in the main crate, but it doesn't hurt to double-check.
#![no_std]

use core::str::FromStr;
use ruci::gui::Go;

fn main() {
    assert_eq!(Go::from_str("go depth 17"), Ok(Go {
        depth: Some(17),
        ..Default::default()
    }));
}