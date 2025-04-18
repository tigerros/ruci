fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::Quit) {
    a(m);
}

fn main() {}