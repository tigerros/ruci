//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a custom position, and let it finish.
//!
//! For an example where calculation is interrupted, see `go_async_info`.
//!
//! This example requires that you have installed Stockfish.
#![cfg(feature = "engine-connection")]
use ruci::gui::Position;
use ruci::EngineConnection;
use ruci::{gui, UciMoves};
use shakmaty::uci::UciMove;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut engine_conn = EngineConnection::from_path("stockfish").unwrap();

    println!("== Sending use UCI message, waiting for uciok");

    let (id, options) = engine_conn.use_uci().await?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");

    println!("== Sending custom FEN with an extra move");

    engine_conn
        .send_message(
            &Position::Fen {
                fen: "rnbqk2r/ppppp1bp/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR w KQkq - 1 5".to_string(),
                moves: Some(UciMoves(vec![UciMove::from_ascii(b"b1c3").unwrap()])),
            }
            .into(),
        )
        .await?;

    println!("== Sending isready message, waiting for readyok");

    engine_conn.is_ready().await?;

    let (infos, best_move) = engine_conn
        .go(gui::Go {
            depth: Some(20),
            ..Default::default()
        })
        .await?;

    for info in infos {
        println!("Info: {info:?}");
    }

    // This should probably be e2g8, but might change depending on how stockfish feels
    println!("Best move: {best_move:?}");

    println!("== Sending quit message");
    engine_conn.send_message(&gui::Quit.into()).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
