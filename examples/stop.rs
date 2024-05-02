use ruci::messages::engine_to_gui::EngineToGuiMessage;
use ruci::messages::gui_to_engine::GoMessage;
use ruci::{GuiToEngineUciConnection, GuiToEngineUciConnectionSync};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let mut uci = GuiToEngineUciConnectionSync::new(
        GuiToEngineUciConnection::new_from_path("stockfish").unwrap(),
    );
    let (info_sender, info_receiver) = mpsc::channel();

    println!(
        "GO: {:#?}",
        uci.go(
            GoMessage {
                search_moves: None,
                ponder: false,
                white_time: None,
                black_time: None,
                white_increment: None,
                black_increment: None,
                moves_to_go: None,
                depth: Some(100),
                nodes: None,
                mate: None,
                move_time: None,
                infinite: false,
            },
            info_sender
        )
    );

    thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            println!("Info: {}", EngineToGuiMessage::Info(info));
        }
    });

    println!("Waiting");
    thread::sleep(Duration::from_secs(3));
    println!("Aborting");
    uci.abort_thread().unwrap();
    println!("Aborted");
    thread::sleep(Duration::from_secs(100));
}
