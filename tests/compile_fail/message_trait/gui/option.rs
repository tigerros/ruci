fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::Option) {
    a(m);
}

fn main() {}