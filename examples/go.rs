//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a position, and let it finish.
//!
//! For an example where calculation is interrupted, see `go_async_info`.
//!
//! This example requires that you have installed Stockfish.

use ruci::messages::{GoMessage, GuiMessage};
use ruci::EngineConnection;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut engine_conn = EngineConnection::from_path("stockfish").unwrap();

    println!("== Sending use UCI message, waiting for uciok");

    let (id, options) = engine_conn.use_uci().await?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");
    println!("== Sending isready message, waiting for readyok");

    let (infos, best_move) = engine_conn
        .go(GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: Some(20),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        })
        .await?;

    for info in infos {
        println!("Info: {info:?}");
    }

    println!("Best move: {best_move:?}");

    println!("== Sending quit message");
    engine_conn.send_message(&GuiMessage::Quit).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
