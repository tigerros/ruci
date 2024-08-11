//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a position, but interrupt it after 2 seconds.
//!
//! For an example where calculation is finished (and the `go_async_info` function is not used),
//! see `go`.
//!
//! This example requires that you have installed Stockfish.

use parking_lot::Mutex;
use ruci::messages::{GoMessage, GuiMessage};
use ruci::EngineConnection;
use std::io;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut engine_conn = EngineConnection::from_path("stockfish").unwrap();

    println!("== Sending use UCI message, waiting for uciok");

    let (id, options) = engine_conn.use_uci().await?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");
    println!("== Sending isready message, waiting for readyok");

    let engine_conn = Arc::new(Mutex::new(engine_conn));

    let (mut info_rx, handle) = EngineConnection::go_async_info(
        engine_conn.clone(),
        GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: None,
            nodes: None,
            mate: None,
            move_time: None,
            infinite: true,
        },
    );

    tokio::spawn(async move {
        while let Some(info) = info_rx.recv().await {
            println!("Info: {info:?}");
        }
    });

    println!("== Waiting 2 secs");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("== Aborting task");
    handle.abort();
    println!("== Aborted");
    println!("== Task result: {:#?}", handle.await);
    println!("== Sending quit message");

    let mut engine_conn = Arc::into_inner(engine_conn).unwrap().into_inner();
    engine_conn.send_message(&GuiMessage::Quit).await?;

    println!("== Sent. Program terminated");

    Ok(())
}
