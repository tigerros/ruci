fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::engine::Message) {
    a(m);
}

fn main() {}