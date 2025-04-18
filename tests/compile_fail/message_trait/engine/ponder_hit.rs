fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::PonderHit) {
    a(m);
}

fn main() {}