//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a position, but interrupt it after a couple of seconds.
//!
//! For an example where calculation is finished (and the `go_async_info` function is not used),
//! see `go`.
//!
//! This example requires that you have installed Stockfish.
#![cfg(feature = "engine-connection")]
use parking_lot::Mutex;
use ruci::gui;
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

    engine_conn.is_ready().await?;

    println!("== Received readyok, starting analysis");

    let engine_conn = Arc::new(Mutex::new(engine_conn));

    let (mut info_rx, handle) = EngineConnection::go_async_info(
        engine_conn.clone(),
        gui::Go {
            infinite: true,
            ..Default::default()
        },
    );

    tokio::spawn(async move {
        while let Some(info) = info_rx.recv().await {
            println!("Info: {info:?}");
        }
    });

    println!("== Waiting 5 secs");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("== Aborting task");
    handle.abort();
    println!("== Aborted");
    println!("== Task result: {:#?}", handle.await);
    println!("== Sending quit message");

    engine_conn
        .lock_arc()
        .send_message(&gui::Message::Quit)
        .await?;

    println!("== Sent. Program terminated");

    Ok(())
}
