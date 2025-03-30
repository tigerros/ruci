#![no_std]

use core::str::FromStr;
use ruci::gui::Go;

fn main() {
    assert_eq!(
        Go::from_str("go depth 17"),
        Ok(Go {
            depth: Some(17),
            ..Default::default()
        })
    );
}
