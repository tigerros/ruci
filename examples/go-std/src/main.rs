use ruci::engine::Info;
use ruci::gui;
use ruci::gui::Position;
use ruci::Engine;
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use std::sync::mpsc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::from_path("stockfish")?;

    println!("== Sending uci, waiting for uciok");

    let (id, options) = engine.use_uci()?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");

    println!("== Sending custom FEN with an extra move");

    engine.send(
        &Position::Fen {
            fen: Fen::from_ascii(
                b"rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5",
            )?,
            moves: vec![UciMove::from_ascii(b"b1c3")?],
        }
        .into(),
    )?;

    println!("== Sending isready, waiting for readyok");

    engine.is_ready()?;

    println!("== Received readyok, sending go");

    let mut infos = Vec::new();
    let best_move = engine.go(
        gui::Go {
            depth: Some(20),
            ..Default::default()
        },
        |info| infos.push(info),
    )?;

    println!("Received {} infos", infos.len());
    println!("Best move (probably e2g8): {best_move:?}");

    println!("== Sending go again, this time receiving and printing them async");

    let (info_sender, info_receiver) = mpsc::channel::<Info>();

    let info_printing_thread = thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            let mut info = info.to_string();
            info.truncate(20);
            println!("Info: {info}");
        }
    });

    let _ = engine.go(
        gui::Go {
            depth: Some(20),
            ..Default::default()
        },
        |info| {
            let _ = info_sender.send(info);
        },
    )?;

    drop(info_sender);

    println!("== Sending quit");
    engine.send(&gui::Quit.into())?;
    println!("== Sent. Waiting for info printing thread to finish");
    info_printing_thread.join().unwrap();
    println!("== Program terminated");

    Ok(())
}
