use crate::messages::engine_to_gui::{BestMoveMessage, EngineToGuiMessage, EngineToGuiMessageParameterPointer, EngineToGuiMessagePointer, IdMessageKind, OptionMessage};
use std::io;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::io::{Write, Read};
use crate::messages::gui_to_engine::{GoMessage, GuiToEngineMessage};
use crate::{Message, MessageParameterPointer, MessageParseError, RawUciMessage};
use std::str::FromStr;

pub struct GuiToEngineUciConnection {
    pub process: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
}

pub struct EngineToGuiUciConnection {
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
    Spawn(io::Error),
    StdoutIsNone,
    StdinIsNone
}

pub enum UciReadMessageError<MessageParameterPtr> where MessageParameterPtr: MessageParameterPointer {
    Read(io::Error),
    MessageParse(MessageParseError<MessageParameterPtr>)
}

pub trait UciConnection<MSend, MReceive> where Self: Sized, MSend: Message, MReceive: Message {
    fn new(process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self;
    fn process(&self) -> &Child;
    fn process_mut(&mut self) -> &mut Child;
    fn stdout(&self) -> &ChildStdout;
    fn stdout_mut(&mut self) -> &mut ChildStdout;
    fn stdin(&self) -> &ChildStdin;
    fn stdin_mut(&mut self) -> &mut ChildStdin;
    fn new_from_path(path: &str) -> Result<Self, UciCreationError> {
        let mut process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().map_err(UciCreationError::Spawn)?;

        println!("spawned it");

        let Some(stdout) = process.stdout.take() else {
            return Err(UciCreationError::StdoutIsNone);
        };

        println!("took stdout");

        let Some(stdin) = process.stdin.take() else {
            return Err(UciCreationError::StdinIsNone);
        };

        println!("tookstdin");

        Ok(Self::new(process, stdout, stdin))
    }

    fn send_message(&mut self, message: GuiToEngineMessage) -> io::Result<()> {
        self.stdin_mut().write_all(message.to_string().as_bytes())
    }

    fn skip_messages(&mut self, count: usize) -> io::Result<()> {
        let mut buf = [0; 1];
        let mut skipped_count = 0;

        loop {
            self.stdout_mut().read_exact(&mut buf)?;

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

    /// Reads a line and attempts to parse it into a message.
    ///
    /// # Errors
    ///
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    fn read_message(&mut self) -> Result<MReceive, UciReadMessageError<MReceive::MessageParameterPointer>> {
        MReceive::from_str(&self.read_line().map_err(UciReadMessageError::Read)?).map_err(UciReadMessageError::MessageParse)
    }

    /// Reads one line without the trailing `'\n'` character.
    ///
    /// # Errors
    ///
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    fn read_line(&mut self) -> io::Result<String> {
        let mut s = String::with_capacity(100);
        let mut buf = [0; 1];

        loop {
            self.stdout_mut().read_exact(&mut buf)?;

            if buf[0] == b'\n' {
                break;
            }

            s.push(char::from(buf[0]));
        }

        Ok(s)
    }
}

impl UciConnection<GuiToEngineMessage, EngineToGuiMessage> for GuiToEngineUciConnection {
    fn new(process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self {
        Self { process, stdout, stdin }
    }
    fn process(&self) -> &Child {
        &self.process
    }
    fn process_mut(&mut self) -> &mut Child {
        &mut self.process
    }
    fn stdout(&self) -> &ChildStdout {
        &self.stdout
    }
    fn stdout_mut(&mut self) -> &mut ChildStdout {
        &mut self.stdout
    }
    fn stdin(&self) -> &ChildStdin {
        &self.stdin
    }
    fn stdin_mut(&mut self) -> &mut ChildStdin {
        &mut self.stdin
    }
}

impl UciConnection<EngineToGuiMessage, GuiToEngineMessage> for EngineToGuiUciConnection {
    fn new(process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self {
        Self { process, stdout, stdin }
    }
    fn process(&self) -> &Child {
        &self.process
    }
    fn process_mut(&mut self) -> &mut Child {
        &mut self.process
    }
    fn stdout(&self) -> &ChildStdout {
        &self.stdout
    }
    fn stdout_mut(&mut self) -> &mut ChildStdout {
        &mut self.stdout
    }
    fn stdin(&self) -> &ChildStdin {
        &self.stdin
    }
    fn stdin_mut(&mut self) -> &mut ChildStdin {
        &mut self.stdin
    }
}

impl GuiToEngineUciConnection {
    /// Sends the [`GuiToEngineMessage::UseUci`] message and returns the engine's ID and a vector of options
    /// once the `uciok` message is received.
    /// 
    /// # Errors
    /// 
    /// See [`Write::write_all`].
    pub fn use_uci(&mut self) -> io::Result<(Option<IdMessageKind>, Vec<OptionMessage>)> {
        self.send_message(GuiToEngineMessage::UseUci)?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<IdMessageKind>;

        loop {
            let Ok(engine_to_gui_message) = self.read_message() else { continue };

            match engine_to_gui_message {
                EngineToGuiMessage::Option(option) => options.push(option),
                EngineToGuiMessage::Id(new_id) => {
                    if let Some(old_id) = id {
                        id = Some(match (old_id, new_id) {
                            (IdMessageKind::Author(author), IdMessageKind::Author(_)) => IdMessageKind::Author(author),
                            (IdMessageKind::Name(name), IdMessageKind::Name(_)) => IdMessageKind::Name(name),
                            (IdMessageKind::Author(author), IdMessageKind::Name(name)) => IdMessageKind::NameAndAuthor(name, author),
                            (IdMessageKind::Name(name), IdMessageKind::Author(author)) => IdMessageKind::NameAndAuthor(name, author),
                            (IdMessageKind::NameAndAuthor(name, author), _) => IdMessageKind::NameAndAuthor(name, author),
                            (IdMessageKind::Author(author), IdMessageKind::NameAndAuthor(name, _)) => IdMessageKind::NameAndAuthor(name, author),
                            (IdMessageKind::Name(name), IdMessageKind::NameAndAuthor(_, author)) => IdMessageKind::NameAndAuthor(name, author)
                        });
                    } else {
                        id = Some(new_id);
                    }
                },
                EngineToGuiMessage::UciOk => return Ok((id, options)),
                _ => (),
            }
        }
    }

    /// Sends the `go` message to the engine and waits for the `bestmove` message response.
    /// Note that the engine sends a bunch of other messages before sending the final `bestmove`
    /// message. If you want to read those messages too,
    /// you should manually send the message with `self.stdin.write_all` and continuously read messages with [`Self::read_line`].
    pub fn go(&mut self, message: GoMessage) -> Result<BestMoveMessage, UciError> {
        self.send_message(GuiToEngineMessage::Go(message)).map_err(UciError::Write)?;

        loop {
            let engine_to_gui_message = match self.read_message() {
                Ok(msg) => msg,
                Err(UciReadMessageError::Read(e)) => return Err(UciError::Read(e)),
                Err(_) => continue,
            };

            if let EngineToGuiMessage::BestMove(best_move) = engine_to_gui_message {
                return Ok(best_move);
            }
        }
    }

    /// Sends the `isready` message and waits for the `readyok` response.
    /// 
    /// # Errors
    /// 
    /// See [`Write::write_all`].
    pub fn isready(&mut self) -> io::Result<()> {
        self.send_message(GuiToEngineMessage::IsReady)?;
        
        loop {
            match self.read_message() {
                Ok(EngineToGuiMessage::ReadyOk) => return Ok(()),
                Ok(_) | Err(_) => continue,
            }
        }
    }


}