use crate::messages::engine_to_gui::{
    BestMoveMessage, EngineToGuiMessage, IdMessageKind, InfoMessage, OptionMessage,
};
use crate::messages::gui_to_engine::{GoMessage, GuiToEngineMessage};
use crate::{Message, MessageParameterPointer, MessageParseError};
use std::io;
use std::io::{Read, Write};
use std::os::windows::process::CommandExt;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

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
pub enum UciCreationError {
    Spawn(io::Error),
    StdoutIsNone,
    StdinIsNone,
}

pub enum UciReadMessageError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    Read(io::Error),
    MessageParse(MessageParseError<MessageParameterPtr>),
}

pub trait UciConnection<MSend, MReceive>
where
    Self: Sized,
    MSend: Message,
    MReceive: Message,
{
    fn new(process: Child, stdout: ChildStdout, stdin: ChildStdin) -> Self;
    fn process(&self) -> &Child;
    fn process_mut(&mut self) -> &mut Child;
    fn stdout(&self) -> &ChildStdout;
    fn stdout_mut(&mut self) -> &mut ChildStdout;
    fn stdin(&self) -> &ChildStdin;
    fn stdin_mut(&mut self) -> &mut ChildStdin;
    /// Creates a new UCI connection from the given executable path.
    ///
    /// # Errors
    ///
    /// - Spawning the process errored.
    /// - Stdout is [`None`].
    /// - Stdin is [`None`].
    fn new_from_path(path: &str) -> Result<Self, UciCreationError> {
        let mut cmd = Command::new(path);
        let mut cmd = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        
        if cfg!(windows) {
            // CREATE_NO_WINDOW
            // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
            cmd = cmd.creation_flags(0x08000000);
        }
        
        let mut process = cmd.spawn()
            .map_err(UciCreationError::Spawn)?;

        let Some(stdout) = process.stdout.take() else {
            return Err(UciCreationError::StdoutIsNone);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(UciCreationError::StdinIsNone);
        };

        Ok(Self::new(process, stdout, stdin))
    }

    /// Sends a message.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    fn send_message(&mut self, message: GuiToEngineMessage) -> io::Result<()> {
        self.stdin_mut().write_all(message.to_string().as_bytes())
    }

    /// Skips a number of lines.
    ///
    /// # Errors
    ///
    /// See [`Read::read_exact`].
    fn skip_lines(&mut self, count: usize) -> io::Result<()> {
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
    fn read_message(
        &mut self,
    ) -> Result<MReceive, UciReadMessageError<MReceive::MessageParameterPointer>> {
        MReceive::from_str(&self.read_line().map_err(UciReadMessageError::Read)?)
            .map_err(UciReadMessageError::MessageParse)
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
        Self {
            process,
            stdout,
            stdin,
        }
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
        Self {
            process,
            stdout,
            stdin,
        }
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
            let Ok(engine_to_gui_message) = self.read_message() else {
                continue;
            };

            match engine_to_gui_message {
                EngineToGuiMessage::Option(option) => options.push(option),
                EngineToGuiMessage::Id(new_id) => {
                    if let Some(old_id) = id {
                        id = Some(match (old_id, new_id) {
                            (IdMessageKind::Author(author), IdMessageKind::Author(_)) => {
                                IdMessageKind::Author(author)
                            }
                            (IdMessageKind::Name(name), IdMessageKind::Name(_)) => {
                                IdMessageKind::Name(name)
                            }
                            (
                                IdMessageKind::Author(author),
                                IdMessageKind::Name(name) | IdMessageKind::NameAndAuthor(name, _),
                            )
                            | (
                                IdMessageKind::Name(name),
                                IdMessageKind::Author(author)
                                | IdMessageKind::NameAndAuthor(_, author),
                            )
                            | (IdMessageKind::NameAndAuthor(name, author), _) => {
                                IdMessageKind::NameAndAuthor(name, author)
                            }
                        });
                    } else {
                        id = Some(new_id);
                    }
                }
                EngineToGuiMessage::UciOk => return Ok((id, options)),
                _ => (),
            }
        }
    }

    /// Sends the `go` message to the engine and waits for the `bestmove` message response,
    /// returning it, along with a list of `info` messages.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub fn go(&mut self, message: GoMessage) -> io::Result<(Vec<InfoMessage>, BestMoveMessage)> {
        let mut info_messages = Vec::<InfoMessage>::with_capacity(
            message.depth.map_or(100, |depth| depth.saturating_add(3)),
        );

        self.send_message(GuiToEngineMessage::Go(message))?;

        loop {
            let engine_to_gui_message = match self.read_message() {
                Ok(msg) => msg,
                Err(UciReadMessageError::Read(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                EngineToGuiMessage::Info(info) => info_messages.push(*info),
                EngineToGuiMessage::BestMove(best_move) => return Ok((info_messages, best_move)),
                _ => (),
            }
        }
    }

    /// Sends the `isready` message and waits for the `readyok` response.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading until `readyok`) errored.
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
