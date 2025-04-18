fn a(_: impl ruci::gui::traits::Message) {}
fn b(m: ruci::BestMove) {
    a(m);
}

fn main() {}