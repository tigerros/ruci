fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Message) {
    a(m);
}

fn main() {}