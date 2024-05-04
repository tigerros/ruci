use ruci::messages::engine_to_gui::EngineToGuiMessage;
use ruci::messages::gui_to_engine::GoMessage;
use ruci::{GuiToEngineUciConnection, GuiToEngineUciConnectionGo};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let uci = Arc::new(Mutex::new(
        GuiToEngineUciConnection::new_from_path("stockfish").unwrap(),
    ));

    let GuiToEngineUciConnectionGo {
        stop,
        info_receiver,
        thread,
    } = GuiToEngineUciConnection::go_async(
        uci,
        GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: Some(10),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        },
    );

    thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            println!("Info: {}", EngineToGuiMessage::Info(info));
        }
    });

    println!("Waiting");
    thread::sleep(Duration::from_secs(3));
    println!("isfinished: {}", thread.is_finished());
    println!("Aborting");
    stop().unwrap();
    println!("Res: {:#?}", thread.join());
    println!("Aborted");
    thread::sleep(Duration::from_secs(100));
}
