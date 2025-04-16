extern crate test;
use pretty_assertions::assert_eq;
use std::str::FromStr;
use test::Bencher;

const INFO: &str = "info depth 20 seldepth 31 time 12 nodes 4 pv e2e4 c7c5 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5";

#[test]
fn equals() {
    let ruci_to_str = ruci::Message::from_str(INFO).unwrap().to_string();
    let shakmaty_uci_to_str = shakmaty_uci::UciMessage::from_str(INFO)
        .unwrap()
        .to_string();
    let vampirc_to_str = vampirc_uci::parse(INFO).first().unwrap().to_string();

    assert_eq!(ruci_to_str, shakmaty_uci_to_str);
    assert_eq!(ruci_to_str, vampirc_to_str);
}

#[bench]
fn ruci(b: &mut Bencher) {
    b.iter(|| ruci::engine::Info::from_str(INFO).unwrap());
}

#[bench]
fn shakmaty_uci(b: &mut Bencher) {
    b.iter(|| shakmaty_uci::UciMessage::from_str(INFO).unwrap());
}

#[bench]
fn vampirc(b: &mut Bencher) {
    b.iter(|| vampirc_uci::parse(INFO).swap_remove(0));
}
