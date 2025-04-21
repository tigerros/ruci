//! This example shows how to:
//! - Start a UCI engine connection.
//! - Send it some initial commands.
//! - Analyze a custom position.
//! - Analyze a custom position and receive info messages on a separate thread.
//!
//! Note that this will print out the (truncated) `Display` impls of the `Info` messages.
//! That is not a reading from the engine, those are parsed messages converted back into a string
//! because it is easier to read.
//!
//! Requires that you have installed Stockfish.

use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use std::borrow::Cow;
use std::io::BufReader;
use std::process::{ChildStdin, ChildStdout};
use std::sync::mpsc;
use std::thread;

fn main() -> anyhow::Result<()> {
    let mut engine =
        ruci::Engine::<BufReader<ChildStdout>, ChildStdin>::from_path("stockfish", false)?;

    println!("== Sending uci, waiting for uciok");

    let mut options = Vec::new();
    let id = engine.use_uci(|option| options.push(option))?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");

    println!("== Sending custom FEN with an extra move");

    // You can also use a reference
    engine.send(ruci::Position::Fen {
        fen: Cow::Owned(Fen::from_ascii(
            b"rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5",
        )?),
        moves: Cow::Borrowed(&[UciMove::from_ascii(b"b1c3")?]),
    })?;

    println!("== Sending isready, waiting for readyok");

    engine.is_ready()?;

    println!("== Received readyok, sending go");

    let mut infos = Vec::new();
    let best_move = engine.go(
        &ruci::Go {
            depth: Some(20),
            ..Default::default()
        },
        |info| infos.push(info),
    )?;

    println!("Received {} infos", infos.len());
    println!("Best move (probably e2g8): {best_move:?}");

    println!("== Sending go again, this time receiving and printing them async");

    let (info_sender, info_receiver) = mpsc::channel::<ruci::Info>();

    let info_printing_thread = thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            let mut info = info.to_string();
            info.truncate(20);
            println!("Info: {info}");
        }
    });

    let _ = engine.go(
        &ruci::Go {
            depth: Some(20),
            ..Default::default()
        },
        |info| {
            let _ = info_sender.send(info);
        },
    )?;

    drop(info_sender);

    println!("== Sending quit");
    engine.send(ruci::Quit)?;
    println!("== Sent. Waiting for info printing thread to finish");
    info_printing_thread.join().unwrap();
    println!("== Program terminated");

    Ok(())
}
