use crate::messages::engine_to_gui::{BestMoveMessage, EngineToGuiMessage, EngineToGuiMessageParameterPointer, EngineToGuiMessagePointer};
use std::io;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::io::{Write, Read};
use crate::messages::gui_to_engine::{GoMessage, GuiToEngineMessage, GuiToEngineMessagePointer};
use crate::{MessageTryFromRawUciMessageError, RawUciMessage, RawUciMessageParseError};
use std::str::FromStr;

pub struct UciConnection {
    pub process: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
}

#[derive(Debug)]
pub enum UciError {
    Write(io::Error),
    Read(io::Error)
}

#[derive(Debug)]
pub enum UciCreationError {
    Io(io::Error),
    StdoutIsNone,
    StdinIsNone
}

impl UciConnection {
    pub fn new(engine_path: &str) -> Result<Self, UciCreationError> {
        let mut cmd = Command::new(engine_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().map_err(UciCreationError::Io)?;

        let Some(stdout) = cmd.stdout.take() else {
            return Err(UciCreationError::StdoutIsNone);
        };

        let Some(stdin) = cmd.stdin.take() else {
            return Err(UciCreationError::StdinIsNone);
        };

        Ok(Self { process: cmd, stdout, stdin })
    }
    
    pub fn skip_messages(&mut self, count: usize) -> Result<(), io::Error> {
        let mut buf = [0; 1];
        let mut skipped_count = 0;
        
        loop {
            self.stdout.read_exact(&mut buf)?;

            if buf[0] == b'\n' {
                skipped_count += 1;
                
                if skipped_count == count {
                    break;
                }
                
                continue;
            }
        }
        
        Ok(())
    }

    /// Reads one line without the trailing `'\n'` character.
    ///
    /// # Errors
    ///
    /// - `self.process.stdout` is [`None`].
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    pub fn read_line(&mut self) -> Result<String, io::Error> {
        let mut s = String::with_capacity(100);
        let mut buf = [0; 1];

        loop {
            self.stdout.read_exact(&mut buf)?;

            if buf[0] == b'\n' {
                break;
            }

            s.push(char::from(buf[0]));
        }
        
        Ok(s)
    }
    
    /// Sends the [`GuiToEngineMessage::UseUci`] message.
    /// 
    /// # Errors
    /// 
    /// See [`Write::write_all`].
    pub fn use_uci(&mut self) -> io::Result<()> {
        self.stdin.write_all(GuiToEngineMessage::UseUci.to_string().as_bytes())
    }

    /// Sends the `go` message to the engine and waits for the `bestmove` message response.
    /// Note that the engine sends a bunch of other messages before sending the final `bestmove`
    /// message. If you want to read those messages too,
    /// you should manually send the message with `self.stdin.write_all` and continuously read messages with [`Self::read_line`].
    pub fn go(&mut self, message: GoMessage) -> Result<BestMoveMessage, UciError> {
        self.stdin.write_all(GuiToEngineMessage::Go(message).to_string().as_bytes()).map_err(UciError::Write)?;

        loop {
            let line = self.read_line().map_err(UciError::Read)?;

            let Ok(raw_uci_message) = RawUciMessage::<EngineToGuiMessagePointer, EngineToGuiMessageParameterPointer>::from_str(&line) else {
                continue
            };

            let Ok(engine_to_gui_message) = EngineToGuiMessage::try_from(raw_uci_message) else {
                continue;
            };

            if let EngineToGuiMessage::BestMove(best_move) = engine_to_gui_message {
                return Ok(best_move);
            }
        }
    }
}