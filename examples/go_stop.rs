//! This example shows how to start a UCI connection, send it some initial commands,
//! and then start calculating a position, but interrupt it after 3 seconds.
//!
//! This example requires that you have installed stockfish (I have used stockfish 16.1).
//!
//! Output on my machine can be found on [pastebin](https://pastebin.com/uF91FKGL).
//! Do note that this is what should "roughly" be the output, I might not update it every time I update the crate.

use parking_lot::Mutex;
use ruci::messages::{GoMessage, GuiMessage};
use ruci::{EngineConnection, GuiToEngineUciConnectionGo};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let uci = Arc::new(Mutex::new(
        EngineConnection::new_from_path("stockfish").unwrap(),
    ));

    println!("Sending use UCI message, waiting for uciok");
    let (id, options) = uci.lock_arc().use_uci().unwrap();
    println!("Received uciok");
    println!("ID: {id:#?}");
    println!("Options: {options:#?}");
    println!("Sending isready message, waiting for readyok");
    uci.lock_arc().is_ready().unwrap();
    println!("Received readyok");

    let GuiToEngineUciConnectionGo {
        stop,
        info_receiver,
        thread,
    } = EngineConnection::go_async(
        uci.clone(),
        GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: Some(30),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        },
    )
    .unwrap();

    thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            println!("Info: {info:#?}");
        }
    });

    println!("Waiting");
    thread::sleep(Duration::from_secs(3));
    println!("Aborting");
    stop().unwrap();
    println!("Aborted");
    println!("Thread result: {:#?}", thread.join());
    println!("Sending quit message");
    uci.lock_arc().send_message(&GuiMessage::Quit).unwrap();
    println!("Sent");
    thread::sleep(Duration::from_secs(100));
}
