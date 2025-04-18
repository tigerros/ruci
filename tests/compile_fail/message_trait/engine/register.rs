fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Register) {
    a(m);
}

fn main() {}