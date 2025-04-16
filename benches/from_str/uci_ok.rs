extern crate test;
use pretty_assertions::assert_eq;
use std::str::FromStr;
use test::Bencher;

const UCI_OK: &str = "uciok\n";

#[test]
fn equals() {
    let ruci_to_str = ruci::Message::from_str(UCI_OK).unwrap().to_string();
    let shakmaty_uci_to_str = shakmaty_uci::UciMessage::from_str(UCI_OK)
        .unwrap()
        .to_string();
    let vampirc_to_str = vampirc_uci::parse(UCI_OK).first().unwrap().to_string();

    assert_eq!(ruci_to_str, shakmaty_uci_to_str);
    assert_eq!(ruci_to_str, vampirc_to_str);
}

#[bench]
fn ruci(b: &mut Bencher) {
    b.iter(|| ruci::engine::UciOk::from_str(UCI_OK).unwrap());
}

#[bench]
fn shakmaty_uci(b: &mut Bencher) {
    b.iter(|| shakmaty_uci::UciMessage::from_str(UCI_OK).unwrap());
}

#[bench]
fn vampirc(b: &mut Bencher) {
    b.iter(|| vampirc_uci::parse(UCI_OK).swap_remove(0));
}
