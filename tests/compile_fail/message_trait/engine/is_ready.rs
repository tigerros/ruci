fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::IsReady) {
    a(m);
}

fn main() {}