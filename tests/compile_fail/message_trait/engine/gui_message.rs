fn a(_: impl ruci::engine::traits::Message) {}
fn b(m: ruci::gui::Message) {
    a(m);
}

fn main() {}