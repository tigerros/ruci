use std::io::{stdin, stdout};
use engine::engine;

fn main() {
    engine(stdout().lock(), stdin().lock()).unwrap();
}
