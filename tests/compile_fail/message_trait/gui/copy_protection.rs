fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::CopyProtection) {
    a(m);
}

fn main() {}