//! This example shows how to start a UCI connection, send it some initial commands,
//! and then start calculating a position, but interrupt it after 3 seconds.
//!
//! This example requires that you have installed stockfish (I have used stockfish 16.1).
//!
//! Output on my machine can be found on [pastebin](https://pastebin.com/uF91FKGL).
//! Do note that this is what should "roughly" be the output, I might not update it every time I update the crate.

use ruci::{
    messages::{GoMessage, GuiMessage},
    EngineConnection,
};
use std::time::Duration;
use tokio::{join, task};

#[tokio::main]
async fn main() {
    let uci = EngineConnection::from_path("stockfish").unwrap();

    println!("Sending `uci` and `isready` messages.");

    let use_uci_fut = uci.use_uci();
    let is_ready_fut = uci.is_ready();
    let (id, options) = join!(use_uci_fut, is_ready_fut).0.unwrap();

    println!("Received `uciok` and `readyok`.");
    println!("ID: {id:?}");
    println!("Options: {options:?}");
    println!("Starting `go` task.");

    let (mut info_receiver, go_task) = uci.go_task(GoMessage {
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
    });

    task::spawn(async move {
        while let Some(info) = info_receiver.recv().await {
            println!("Info: {info:?}");
        }
    });

    println!("Waiting 2 secs.");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Aborting.");
    go_task.abort();
    println!("Aborted.");
    println!("Task result: {:?}", go_task.await);

    println!("Sending quit message.");
    uci.send_message(&GuiMessage::Quit).await.unwrap();
    println!("Sent. Program finished!");
    tokio::time::sleep(Duration::from_secs(100)).await;
}
