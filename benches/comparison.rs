use criterion::{criterion_group, criterion_main, Criterion};
use ruci::UciMoves;
use shakmaty::uci::UciMove;
use std::hint::black_box;
use std::str::FromStr;
use vampirc_uci::UciSquare;

fn to_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_str");
    // These are bogus values they're not sent to an engine
    let ruci_go = ruci::gui::Go {
        search_moves: Some(UciMoves(vec![
            UciMove::from_ascii(b"e2e4").unwrap(),
            UciMove::from_ascii(b"g2g4").unwrap(),
        ])),
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
    };
    let shakmaty_uci_go = shakmaty_uci::UciMessage::Go {
        time_control: Some(shakmaty_uci::UciTimeControl::Ponder),
        search_control: Some(shakmaty_uci::UciSearchControl {
            search_moves: vec![
                UciMove::from_ascii(b"e2e4").unwrap(),
                UciMove::from_ascii(b"g2g4").unwrap(),
            ],
            mate: Some(0),
            depth: Some(20),
            nodes: Some(57457),
        }),
    };
    let vampirc_go = vampirc_uci::UciMessage::Go {
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
    };

    group.bench_function("ruci_to_str", |b| b.iter(|| black_box(ruci_go.to_string())));
    group.bench_function("shakmaty_uci_to_str", |b| {
        b.iter(|| black_box(shakmaty_uci_go.to_string()))
    });
    group.bench_function("vampirc_uci_to_str", |b| {
        b.iter(|| black_box(vampirc_go.to_string()))
    });
}

fn from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str");
    let str = "info depth 20 seldepth 31 time 12 nodes 4 pv e2e4 c7c5 multipv 1 score cp 22 lowerbound currmove e2e4 tbhits 2 string blabla refutation g2g4 d7d5 f1g2 currline 1 e2e4 c7c5";

    println!(
        "ruci back to string: {}",
        ruci::Message::from_str(str).unwrap()
    );
    println!(
        "shakmaty-uci back to string: {}",
        shakmaty_uci::UciMessage::from_str(str).unwrap()
    );
    println!(
        "vampirc-uci back to string: {}",
        vampirc_uci::parse(str).first().unwrap()
    );

    group.bench_function("ruci_from_str", |b| {
        b.iter(|| black_box(ruci::Message::from_str(str).unwrap()))
    });
    group.bench_function("shakmaty_uci_from_str", |b| {
        b.iter(|| black_box(shakmaty_uci::UciMessage::from_str(str).unwrap()))
    });
    group.bench_function("vampirc_uci_from_str", |b| {
        b.iter(|| {
            // CLIPPY: Intentional!
            #[allow(clippy::unit_arg)]
            black_box(if vampirc_uci::parse(str).is_empty() {
                panic!("at the disco");
            })
        })
    });
}

criterion_group!(benches, to_str, from_str);
criterion_main!(benches);
