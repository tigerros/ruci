fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::Message) {
    a(m);
}

fn main() {}