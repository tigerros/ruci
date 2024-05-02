use ruci::messages::engine_to_gui::EngineToGuiMessage;
use ruci::messages::gui_to_engine::GoMessage;
use ruci::{GuiToEngineUciConnection, GuiToEngineUciConnectionGo};
use std::thread;
use std::time::Duration;

fn main() {
    let uci = GuiToEngineUciConnection::new_from_path("stockfish").unwrap();
    let mut go = GuiToEngineUciConnectionGo::new(uci);

    let (info_receiver, go_handle) = go.go(GoMessage {
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
    });

    thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            println!("Info: {}", EngineToGuiMessage::Info(info));
        }
    });

    //println!("GO: {:#?}", go_handle.join());

    println!("Waiting");
    thread::sleep(Duration::from_secs(3));
    println!("Aborting");
    let _ = go.abort();
    go_handle.join().unwrap();
    println!("Aborted");
    thread::sleep(Duration::from_secs(100));
}
