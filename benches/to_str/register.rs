extern crate test;
use pretty_assertions::assert_eq;
use std::borrow::Cow;
use test::Bencher;

fn ruci_register_borrowed() -> ruci::gui::Register<'static> {
    ruci::gui::Register::NameAndCode {
        name: Cow::Borrowed("Name o46kp2"),
        code: Cow::Borrowed("Code skjjps"),
    }
}

fn ruci_register_owned() -> ruci::gui::Register<'static> {
    ruci::gui::Register::NameAndCode {
        name: Cow::Owned("Name o46kp2".to_string()),
        code: Cow::Owned("Code skjjps".to_string()),
    }
}

fn shakmaty_uci_register() -> shakmaty_uci::UciMessage {
    shakmaty_uci::UciMessage::Register {
        later: false,
        name: Some("Name o46kp2".to_string()),
        code: Some("Code skjjps".to_string()),
    }
}

fn vampirc_register() -> vampirc_uci::UciMessage {
    vampirc_uci::UciMessage::Register {
        later: false,
        name: Some("Name o46kp2".to_string()),
        code: Some("Code skjjps".to_string()),
    }
}

#[test]
fn equals_owned() {
    assert_eq!(
        ruci_register_borrowed().to_string(),
        ruci_register_owned().to_string()
    );
}

#[test]
fn equals_shakmaty() {
    assert_eq!(
        ruci_register_borrowed().to_string(),
        shakmaty_uci_register().to_string()
    );
}

#[test]
fn equals_vampirc() {
    assert_eq!(
        ruci_register_borrowed().to_string(),
        vampirc_register().to_string()
    );
}

#[bench]
fn ruci_borrowed(b: &mut Bencher) {
    b.iter(|| ruci_register_borrowed().to_string());
}

#[bench]
fn ruci_owned(b: &mut Bencher) {
    b.iter(|| ruci_register_owned().to_string());
}

#[bench]
fn shakmaty_uci(b: &mut Bencher) {
    b.iter(|| shakmaty_uci_register().to_string());
}

#[bench]
fn vampirc(b: &mut Bencher) {
    b.iter(|| vampirc_register().to_string());
}
