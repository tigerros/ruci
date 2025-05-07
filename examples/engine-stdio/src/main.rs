use engine::engine;
use std::io::{stdin, stdout};

fn main() {
    engine(stdout().lock(), stdin().lock()).unwrap();
}
