fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Position) {
    a(m);
}

fn main() {}