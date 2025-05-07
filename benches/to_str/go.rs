extern crate test;
use pretty_assertions::assert_eq;
use shakmaty::Square;
use shakmaty::uci::UciMove;
use std::borrow::Cow;
use test::Bencher;
use vampirc_uci::UciSquare;

fn shakmaty_search_moves() -> &'static [UciMove; 2] {
    &[
        UciMove::Normal {
            from: Square::E2,
            to: Square::E4,
            promotion: None,
        },
        UciMove::Normal {
            from: Square::G2,
            to: Square::G4,
            promotion: None,
        },
    ]
}

fn shakmaty_search_moves_vec() -> Vec<UciMove> {
    vec![
        UciMove::Normal {
            from: Square::E2,
            to: Square::E4,
            promotion: None,
        },
        UciMove::Normal {
            from: Square::G2,
            to: Square::G4,
            promotion: None,
        },
    ]
}

fn ruci_go_borrowed() -> ruci::gui::Go<'static> {
    ruci::gui::Go {
        search_moves: Cow::Borrowed(shakmaty_search_moves()),
        ponder: true,
        w_time: None,
        b_time: None,
        w_inc: None,
        b_inc: None,
        moves_to_go: None,
        depth: Some(20),
        nodes: Some(57457),
        mate: Some(0),
        move_time: None,
        infinite: false,
    }
}

fn ruci_go_owned() -> ruci::gui::Go<'static> {
    ruci::gui::Go {
        search_moves: Cow::Owned(shakmaty_search_moves_vec()),
        ponder: true,
        w_time: None,
        b_time: None,
        w_inc: None,
        b_inc: None,
        moves_to_go: None,
        depth: Some(20),
        nodes: Some(57457),
        mate: Some(0),
        move_time: None,
        infinite: false,
    }
}

fn shakmaty_uci_go() -> shakmaty_uci::UciMessage {
    shakmaty_uci::UciMessage::Go {
        time_control: Some(shakmaty_uci::UciTimeControl::Ponder),
        search_control: Some(shakmaty_uci::UciSearchControl {
            search_moves: shakmaty_search_moves_vec(),
            mate: Some(0),
            depth: Some(20),
            nodes: Some(57457),
        }),
    }
}

fn vampirc_go() -> vampirc_uci::UciMessage {
    vampirc_uci::UciMessage::Go {
        time_control: Some(vampirc_uci::UciTimeControl::Ponder),
        search_control: Some(vampirc_uci::UciSearchControl {
            search_moves: vec![
                vampirc_uci::UciMove {
                    from: UciSquare::from('e', 2),
                    to: UciSquare::from('e', 4),
                    promotion: None,
                },
                vampirc_uci::UciMove {
                    from: UciSquare::from('g', 2),
                    to: UciSquare::from('g', 4),
                    promotion: None,
                },
            ],
            mate: Some(0),
            depth: Some(20),
            nodes: Some(57457),
        }),
    }
}

#[test]
fn equals_vec() {
    assert_eq!(ruci_go_borrowed().to_string(), ruci_go_owned().to_string());
}

#[test]
fn equals_shakmaty() {
    // shakmaty_uci produces differently ordered arguments so gotta hardcode it
    assert_eq!(
        ruci_go_borrowed().to_string(),
        "go searchmoves e2e4 g2g4 ponder depth 20 nodes 57457 mate 0".to_string()
    );
    assert_eq!(
        shakmaty_uci_go().to_string(),
        "go ponder depth 20 nodes 57457 mate 0 searchmoves e2e4 g2g4".to_string()
    );
}

#[test]
fn equals_vampirc() {
    // vampirc_uci produces differently ordered arguments so gotta hardcode it
    assert_eq!(
        ruci_go_borrowed().to_string(),
        "go searchmoves e2e4 g2g4 ponder depth 20 nodes 57457 mate 0".to_string()
    );
    // who left these extra spaces in here?? damn vampires and their... liberal sprinkling of whitespace?
    assert_eq!(
        vampirc_go().to_string(),
        "go ponder depth 20 nodes 57457 mate 0  searchmoves e2e4 g2g4 ".to_string()
    );
}

#[bench]
fn ruci_borrowed(b: &mut Bencher) {
    b.iter(|| ruci_go_borrowed().to_string());
}

#[bench]
fn ruci_owned(b: &mut Bencher) {
    b.iter(|| ruci_go_owned().to_string());
}

#[bench]
fn shakmaty_uci(b: &mut Bencher) {
    b.iter(|| shakmaty_uci_go().to_string());
}

#[bench]
fn vampirc(b: &mut Bencher) {
    b.iter(|| vampirc_go().to_string());
}
