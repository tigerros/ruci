fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::SetOption) {
    a(m);
}

fn main() {}