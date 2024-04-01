use std::fmt::{Display, Formatter, Write};
use std::io::Read;
use std::process::Command;
use ruci::{EngineToGuiMessage, EngineToGuiMessageParameterPointer, EngineToGuiMessagePointer, GoMessage, GuiToEngineMessage, GuiToEngineUci, RawUciMessage, SetOptionMessage, SetPositionMessageKind};
use std::str::FromStr;

struct A;

impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hello")?;
        f.write_char('\n')
    }
}

fn main() {
    // let mut cmd = Command::new("stockfish");
    // cmd.arg(""); // Example argument
    // 
    // // Spawn the command and capture its stdout
    // let child = cmd.stdout(Stdio::piped()).spawn().expect("Failed to start command");
    // 
    // // Extract the stdout of the child process
    // let stdout = child.stdout.expect("Failed to open stdout");
    // 
    // // Wrap the stdout in a buffered reader for line-by-line reading
    // let reader = BufReader::new(stdout);
    // 
    // // Iterate over the lines of the child process's stdout
    // for line in reader.lines() {
    //     match line {
    //         Ok(line) => println!("{}", line),
    //         Err(err) => eprintln!("Error reading line: {}", err),
    //     }
    // }
    
    let mut gteuci = GuiToEngineUci::new("stockfish").unwrap();
    
    let mut use_uci = GuiToEngineMessage::UseUci.to_string();
    use_uci.push('\n');
    println!("Sending useuci message: [{use_uci}]");
    gteuci.write_all(&use_uci).unwrap();

    let setoption = GuiToEngineMessage::SetOption(SetOptionMessage {
        name: "Threads".to_string(),
        value: Some("16".to_string()),
    }).to_string();
    println!("Sending setoption message: [{setoption}]");
    gteuci.write_all(&setoption).unwrap();

    let setpos = GuiToEngineMessage::SetPosition(SetPositionMessageKind::StartingPosition { moves: None }).to_string();
    println!("Sending setpos message: [{setpos}]");
    gteuci.write_all(&setpos).unwrap();

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
    }).to_string();
    println!("Sending go message: [{go}]");
    gteuci.write_all(&go).unwrap();

    let response = gteuci.read_line().unwrap();

    println!("Stockfish response: [{response}]");
    
    let response = gteuci.read_line().unwrap();
    
    println!("Stockfish response: [{response}]");
    
    // let raw_message =
    //     RawUciMessage::<EngineToGuiMessagePointer, EngineToGuiMessageParameterPointer>::from_str(
    //         "info depth 10 seldepth 12 multipv 1 score cp 31 nodes 7103 nps 591916 hashfull 4 tbhits 0 time 12 pv d2d4 d7d5 e2e3 g8f6 c2c4 e7e6 g1f3 f8e7 b1c3",
    //     )
    //     .unwrap();
    // println!("{:#?}", raw_message);
    // println!("Message tostring: {raw_message}");
    // 
    // let message = EngineToGuiMessage::try_from(raw_message).unwrap();
    // 
    // if let EngineToGuiMessage::Info(info) = message {
    //     println!("{:#?}", info);
    // }

    //let messages = vampirc_uci::parse("info depth 10 pv d2d4 d7d5 e2e3 g8f6 c2c4 e7e6 g1f3 f8e7 b1c3 seldepth 12 multipv 1 score cp 31 nodes 7103 nps 546384 hashfull 4 tbhits 0 time 13");
    //println!("{:#?}", messages);
}
