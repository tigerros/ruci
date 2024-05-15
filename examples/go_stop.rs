//! This example shows how to start a UCI connection, send it some initial commands,
//! and then start calculating a position, but interrupt it after 3 seconds.
//!
//! This example requires that you have installed stockfish (I have used stockfish 16.1).
//!
//! Output on my machine can be found on [pastebin](https://pastebin.com/uF91FKGL).
//! Do note that this is what should "roughly" be the output, I might not update it every time I update the crate.

use ruci::messages::{GoMessage, GuiMessage};
use ruci::EngineConnection;
use std::process::Stdio;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::io::{stdout, AsyncWriteExt, Stdout};
use tokio::join;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    let uci = Arc::new(Mutex::new(
        EngineConnection::new_from_path("stockfish").unwrap(),
    ));
    let uci_guard = uci.lock().await;

    println!("Sending `uci` and `isready` messages.");
    let use_uci_fut = uci_guard.use_uci();
    let is_ready_fut = uci_guard.is_ready();

    let (id, options) = join!(use_uci_fut, is_ready_fut).0.unwrap();

    println!("Received `uciok` and `readyok`.");
    //println!("ID: {id:#?}");
    //println!("Options: {options:#?}");

    let uci2 = Arc::clone(&uci);

    let go_task = task::spawn(async move {
        let uci_guard = uci2.lock().await;

        println!("Starting go");
        let bestmove = uci_guard.go(GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: Some(5),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        }).await.unwrap().1;
        println!("finished");
        println!("bestmove: {bestmove:#?}");
    });
    
    let res = go_task.await;
    
    println!("Res: {res:#?}");

    // thread::spawn(move || {
    //     while let Ok(info) = info_receiver.recv() {
    //         println!("Info: {info:#?}");
    //     }
    // });

    println!("Waiting 3 secs.");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Aborting.");
    //go_task.abort();
    println!("Aborted.");
    //println!("Task result: {:#?}", go_task.await);
    println!("Sending quit message.");
    uci_guard.send_message(&GuiMessage::Quit).await.unwrap();
    println!("Sent.");
    tokio::time::sleep(Duration::from_secs(100)).await;
}
