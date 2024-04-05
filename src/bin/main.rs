use ruci::messages::engine_to_gui::{
    EngineToGuiMessage, EngineToGuiMessageParameterPointer, EngineToGuiMessagePointer,
};
use ruci::messages::gui_to_engine::{
    GoMessage, GuiToEngineMessage, GuiToEngineMessageParameterPointer, GuiToEngineMessagePointer,
    SetOptionMessage, SetPositionMessageKind,
};
use ruci::{RawUciMessage, UciConnection};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

fn main() {
    let mut conn = UciConnection::new("stockfish").unwrap();

    let use_uci = GuiToEngineMessage::UseUci.to_string();
    conn.write_all(&use_uci).unwrap();

    let setoption = GuiToEngineMessage::SetOption(SetOptionMessage {
        name: "Threads".to_string(),
        value: Some("16".to_string()),
    })
    .to_string();
    conn.write_all(&setoption).unwrap();

    let setpos =
        GuiToEngineMessage::SetPosition(SetPositionMessageKind::StartingPosition { moves: None })
            .to_string();
    conn.write_all(&setpos).unwrap();

    let go = GuiToEngineMessage::Go(GoMessage {
        search_moves: None,
        ponder: false,
        white_time: None,
        black_time: None,
        white_increment: None,
        black_increment: None,
        moves_to_go: None,
        depth: Some(23),
        nodes: None,
        mate: None,
        move_time: None,
        infinite: false,
    })
    .to_string();
    conn.write_all(&go).unwrap();

    // let raw_message_go = RawUciMessage::<
    //     GuiToEngineMessagePointer,
    //     GuiToEngineMessageParameterPointer,
    // >::from_str("go depth 13 ponder nodes 50")
    // .unwrap();
    //
    // let msg_go = GuiToEngineMessage::try_from(raw_message_go).unwrap();
    //
    // println!("{:#?}", msg_go);
    //
    // let raw_message =
    //     RawUciMessage::<EngineToGuiMessagePointer, EngineToGuiMessageParameterPointer>::from_str(
    //         "info depth 10 seldepth 12 multipv 1 score cp 31 nodes 7103 nps 591916 hashfull 4 tbhits 0 time 12 pv d2d4 d7d5 e2e3 g8f6 c2c4 e7e6 g1f3 f8e7 b1c3",
    //     )
    //     .unwrap();
    //
    // let message = EngineToGuiMessage::try_from(raw_message).unwrap();
    //
    // if let EngineToGuiMessage::Info(info) = message.clone() {
    //     println!("{:#?}", info);
    // }
    //
    // println!("info message tostring: {}", message);

    //let messages = vampirc_uci::parse("info depth 10 pv d2d4 d7d5 e2e3 g8f6 c2c4 e7e6 g1f3 f8e7 b1c3 seldepth 12 multipv 1 score cp 31 nodes 7103 nps 546384 hashfull 4 tbhits 0 time 13");
    //println!("{:#?}", messages);
}
