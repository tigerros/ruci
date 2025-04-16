extern crate test;
use pretty_assertions::assert_eq;
use test::Bencher;

#[test]
fn equals_shakmaty() {
    assert_eq!(
        ruci::engine::UciOk.to_string(),
        shakmaty_uci::UciMessage::UciOk.to_string()
    );
}

#[test]
fn equals_vampirc() {
    assert_eq!(
        ruci::engine::UciOk.to_string(),
        vampirc_uci::UciMessage::UciOk.to_string()
    );
}

#[bench]
fn ruci(b: &mut Bencher) {
    b.iter(|| ruci::engine::UciOk.to_string());
}

#[bench]
fn shakmaty_uci(b: &mut Bencher) {
    b.iter(|| shakmaty_uci::UciMessage::UciOk.to_string());
}

#[bench]
fn vampirc(b: &mut Bencher) {
    b.iter(|| vampirc_uci::UciMessage::UciOk.to_string());
}
