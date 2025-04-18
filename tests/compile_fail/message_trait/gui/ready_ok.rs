fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::ReadyOk) {
    a(m);
}

fn main() {}