fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Uci) {
    a(m);
}

fn main() {}