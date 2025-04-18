fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Debug) {
    a(m);
}

fn main() {}