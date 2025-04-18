fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::UciOk) {
    a(m);
}

fn main() {}