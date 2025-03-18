//! This example shows how to start a UCI connection, send it some initial commands,
//! start calculating a position, but interrupt it after a couple of seconds.
//!
//! Note that this will print out the [`Display`](std::fmt::Display) impls of the [`Info`](engine::Info) messages.
//! That is not a reading from the engine, those are parsed messages converted back into a string
//! because it's easier to read.
//!
//! This example requires that you have installed Stockfish.
#![cfg(feature = "engine-connection")]
use ruci::Engine;
use ruci::{engine, gui};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let engine = Arc::new(Mutex::new(Engine::from_path("stockfish", false)?));
    let mut lock = engine.lock().await;

    println!("== Sending use UCI message, waiting for uciok");

    let (id, options) = lock.use_uci().await?;

    println!("== Received uciok");
    println!("== ID: {id:?}");
    println!("== Options: {options:?}");
    println!("== Sending isready message, waiting for readyok");

    lock.is_ready().await?;

    println!("== Received readyok, starting analysis");

    drop(lock);

    let engine2 = engine.clone();
    let infos = Arc::new(Mutex::new(Vec::new()));
    let infos2 = infos.clone();

    let handle = tokio::spawn(async move {
        let mut engine_lock = engine2.lock().await;
        let mut infos_lock = infos2.lock().await;

        let info_fn = move |info: Box<engine::Info>| {
            // Ignore the insignificant ones
            if info.score.is_some() {
                println!("Info #{}: {info}", infos_lock.len());
                infos_lock.push(info);
            }
        };

        engine_lock
            .go_stream(
                gui::Go {
                    infinite: true,
                    ..Default::default()
                },
                info_fn,
            )
            .await
    });

    println!("== Waiting 5 secs");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("== Aborting task");
    handle.abort();
    println!("== Aborted");
    println!("== Task result: {:#?}", handle.await);

    let mut engine = Arc::into_inner(engine).unwrap().into_inner();
    let infos = Arc::into_inner(infos).unwrap().into_inner();

    println!("== Seen {} infos", infos.len());
    println!("== Sending quit message");

    engine.send_message(&gui::Quit.into()).await?;

    println!("== Sent. Program terminated");

    Ok(())
}
