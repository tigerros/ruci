fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Stop) {
    a(m);
}

fn main() {}