fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::Info) {
    a(m);
}

fn main() {}