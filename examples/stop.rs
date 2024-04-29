use std::sync::{Arc, Mutex, mpsc};
use ruci::GuiToEngineUciConnection;
use std::thread;
use std::time::Duration;
use ruci::messages::engine_to_gui::EngineToGuiMessage;
use ruci::messages::gui_to_engine::GoMessage;

fn main() {
    let uci = Arc::new(Mutex::new(GuiToEngineUciConnection::new_from_path("stockfish").unwrap()));
    let uci2 = uci.clone();
    let (info_sender, info_receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut guard = uci2.lock().unwrap();

        guard.go(GoMessage {
            search_moves: None,
            ponder: false,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
            depth: Some(25),
            nodes: None,
            mate: None,
            move_time: None,
            infinite: false,
        }, &info_sender).unwrap();
    });
    
    thread::spawn(move || {
        while let Ok(info) = info_receiver.recv() {
            println!("Info: {}", EngineToGuiMessage::Info(info));
        }
    });
    
    thread::sleep(Duration::from_secs(2));
}