fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::UciNewGame) {
    a(m);
}

fn main() {}