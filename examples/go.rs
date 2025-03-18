//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a custom position, and let it finish.
//!
//! Note that this will print out the [`Display`](std::fmt::Display) impls of the [`Info`](engine::Info) messages.
//! That is not a reading from the engine, those are parsed messages converted back into a string
//! because it's easier to read.
//!
//! This example requires that you have installed Stockfish.
#![cfg(feature = "engine-connection")]

use ruci::gui;
use ruci::gui::Position;
use ruci::Engine;
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut engine = Engine::from_path("stockfish", false)?;

    println!("== Sending use UCI message, waiting for uciok");

    let (id, options) = engine.use_uci().await?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");

    println!("== Sending custom FEN with an extra move");

    engine
        .send(
            &Position {
                startpos: false,
                fen: Some(Fen::from_ascii(
                    b"rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5",
                )?),
                moves: vec![UciMove::from_ascii(b"b1c3").unwrap()],
            }
            .into(),
        )
        .await?;

    println!("== Sending isready message, waiting for readyok");

    engine.is_ready().await?;

    let mut infos = Vec::new();
    let best_move = engine
        .go(
            gui::Go {
                depth: Some(20),
                ..Default::default()
            },
            |info| infos.push(info),
        )
        .await?;

    for info in infos {
        println!("Info: {info}");
    }

    println!("Best move (probably e2g8): {best_move:?}");

    println!("== Sending quit message");
    engine.send(&gui::Quit.into()).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
