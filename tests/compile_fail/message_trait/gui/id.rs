fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::Id) {
    a(m);
}

fn main() {}