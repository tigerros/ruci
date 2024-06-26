use crate::messages::{BestMoveMessage, EngineMessage, IdMessageKind, InfoMessage, OptionMessage};
use crate::messages::{GoMessage, GuiMessage};
use crate::{Message, MessageParameterPointer, MessageParseError};

use std::io;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

#[cfg(feature = "uci-connection-go-async")]
use parking_lot::Mutex;
#[cfg(feature = "uci-connection-go-async")]
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
        mpsc::Receiver,
        Arc,
    },
    thread,
    thread::JoinHandle,
};

/// A connection to an engine, used by a GUI.
pub type EngineConnection = UciConnection<GuiMessage, EngineMessage>;
/// A connection to a GUI, used by an engine.
pub type GuiConnection = UciConnection<EngineMessage, GuiMessage>;

#[derive(Debug)]
/// Something went wrong with spawning the engine process.
pub enum UciCreationError {
    Spawn(io::Error),
    StdoutIsNone,
    StdinIsNone,
}

#[derive(Debug)]
/// Reading the message either resulted in an IO error, or it could not be parsed.
pub enum UciReadMessageError<MessageParameterPtr>
where
    MessageParameterPtr: MessageParameterPointer,
{
    Io(io::Error),
    MessageParse(MessageParseError<MessageParameterPtr>),
}

#[derive(Debug)]
/// This is the basis of a UCI connection; use [`EngineConnection`] or [`GuiConnection`] instead.
pub struct UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    pub process: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
    _phantom: PhantomData<(MSend, MReceive)>,
}

impl<MSend, MReceive> UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    /// Creates a new UCI connection from the given executable path.
    ///
    /// # Errors
    ///
    /// - Spawning the process errored.
    /// - Stdout is [`None`].
    /// - Stdin is [`None`].
    pub fn new_from_path(path: &str) -> Result<Self, UciCreationError> {
        #[cfg(windows)]
        use std::os::windows::process::CommandExt;

        let mut cmd = Command::new(path);
        let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        #[cfg(windows)]
        // CREATE_NO_WINDOW
        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        let cmd = cmd.creation_flags(0x0800_0000);

        let mut process = cmd.spawn().map_err(UciCreationError::Spawn)?;

        let Some(stdout) = process.stdout.take() else {
            return Err(UciCreationError::StdoutIsNone);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(UciCreationError::StdinIsNone);
        };

        Ok(Self {
            process,
            stdout,
            stdin,
            _phantom: PhantomData,
        })
    }

    /// Sends a message.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    pub fn send_message(&mut self, message: &MSend) -> io::Result<()> {
        self.stdin.write_all(message.to_string().as_bytes())
    }

    /// Skips some lines.
    ///
    /// # Errors
    ///
    /// See [`Read::read_exact`].
    pub fn skip_lines(&mut self, count: usize) -> io::Result<()> {
        let mut buf = [0; 1];
        let mut skipped_count = 0;

        loop {
            self.stdout.read_exact(&mut buf)?;

            if buf[0] == b'\n' {
                // CLIPPY: `skipped_count` never overflows because it starts at 0, increments by 1, and stops once `count` is reached.
                #[allow(clippy::arithmetic_side_effects)]
                {
                    skipped_count += 1;
                }

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
    pub fn read_message(
        &mut self,
    ) -> Result<MReceive, UciReadMessageError<MReceive::ParameterPointer>> {
        MReceive::from_str(&self.read_line().map_err(UciReadMessageError::Io)?)
            .map_err(UciReadMessageError::MessageParse)
    }

    /// Reads one line without the trailing `'\n'` character.
    ///
    /// # Errors
    ///
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    pub fn read_line(&mut self) -> io::Result<String> {
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
}

#[cfg(feature = "uci-connection-go-async")]
/// Returned by the [`EngineConnection::go_async`] function.
#[derive(Debug)]
pub struct GuiToEngineUciConnectionGo<Stop>
where
    Stop: FnOnce() -> io::Result<()>,
{
    /// Calling this function sends a "stop" signal to the thread
    /// and also sends the [`stop`](https://backscattering.de/chess/uci/#gui-stop) message to the engine.
    ///
    /// This function does not wait for the `self.thread` to finish.
    ///
    /// # Errors
    ///
    /// - [`GuiToEngineUciConnectionGoError::Poison`]: the [`EngineConnection`] mutex was poisoned.
    /// - [`GuiToEngineUciConnectionGoError::Io`]: see [`UciConnection::send_message`].
    pub stop: Stop,
    /// All [`info`](https://backscattering.de/chess/uci/#engine-info) messages will be sent through this receiver.
    pub info_receiver: Receiver<Box<InfoMessage>>,
    /// This is the handle to the thread, use it to wait for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) message.
    ///
    /// # Errors
    ///
    /// - [`GuiToEngineUciConnectionGoError::Poison`]: the [`EngineConnection`] mutex was poisoned.
    /// - [`GuiToEngineUciConnectionGoError::Io(io::ErrorKind::ConnectionAborted)`](GuiToEngineUciConnectionGoError::Io): the `self.stop` function was called, and the thread aborted.
    /// - [`GuiToEngineUciConnectionGoError::Io`]: write/read IO errors.
    pub thread: JoinHandle<io::Result<BestMoveMessage>>,
}

impl EngineConnection {
    /// Sends the [`GuiMessage::UseUci`] message and returns the engine's ID and a vector of options
    /// once the [`uciok`](https://backscattering.de/chess/uci/#engine-uciok) message is received.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    pub fn use_uci(&mut self) -> io::Result<(Option<IdMessageKind>, Vec<OptionMessage>)> {
        self.send_message(&GuiMessage::UseUci)?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<IdMessageKind>;

        loop {
            let Ok(engine_to_gui_message) = self.read_message() else {
                continue;
            };

            match engine_to_gui_message {
                EngineMessage::Option(option) => options.push(option),
                EngineMessage::Id(new_id) => update_id(&mut id, new_id),
                EngineMessage::UciOk => return Ok((id, options)),
                _ => (),
            }
        }
    }

    /// Sends the [`go`](https://backscattering.de/chess/uci/#gui-go) message to the engine and waits for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) message response,
    /// returning it, along with a list of [`info`](https://backscattering.de/chess/uci/#engine-info) messages.
    ///
    /// See also the [`Self::go_async`] function, which can be interrupted.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub fn go(&mut self, message: GoMessage) -> io::Result<(Vec<InfoMessage>, BestMoveMessage)> {
        let mut info_messages = Vec::<InfoMessage>::with_capacity(
            message.depth.map_or(100, |depth| depth.saturating_add(3)),
        );

        self.send_message(&GuiMessage::Go(message))?;

        loop {
            let engine_to_gui_message = match self.read_message() {
                Ok(msg) => msg,
                Err(UciReadMessageError::Io(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                EngineMessage::Info(info) => info_messages.push(*info),
                EngineMessage::BestMove(best_move) => return Ok((info_messages, best_move)),
                _ => (),
            }
        }
    }

    #[cfg(feature = "uci-connection-go-async")]
    /// Spawns a new thread which sends the [`go`](https://backscattering.de/chess/uci/#gui-go) message to the engine and waits for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) message response,
    /// returning it. The [`info`](https://backscattering.de/chess/uci/#engine-info) messages are sent through the returned receiver.
    ///
    /// See also:
    /// - The [`Self::go`] function for a simpler alternative, but one that cannot be interrupted.
    /// - The [go_stop](https://github.com/tigerros/ruci/tree/master/examples/go_stop.rs) example.
    ///
    /// # Errors
    ///
    /// - Creating the thread failed.
    pub fn go_async(
        arc_self: Arc<Mutex<Self>>,
        message: GoMessage,
    ) -> io::Result<GuiToEngineUciConnectionGo<impl FnOnce() -> io::Result<()>>> {
        let (info_sender, info_receiver) = mpsc::channel();
        let is_running = Arc::new(AtomicBool::new(true));
        let is_running2 = is_running.clone();
        let arc_self2 = arc_self.clone();

        let stop = move || {
            is_running2.store(false, Ordering::SeqCst);
            arc_self2.lock_arc().send_message(&GuiMessage::Stop)?;

            Ok(())
        };

        let thread = thread::Builder::new()
            .name(format!(
                "go_{}",
                message
                    .to_string()
                    .replace(|c: char| c.is_whitespace(), "_")
            ))
            .spawn(move || {
                if !is_running.load(Ordering::SeqCst) {
                    return Err(io::ErrorKind::ConnectionAborted.into());
                }

                let mut guard = arc_self.lock_arc();

                guard.send_message(&GuiMessage::Go(message))?;

                loop {
                    if !is_running.load(Ordering::SeqCst) {
                        return Err(io::ErrorKind::ConnectionAborted.into());
                    };

                    let message = guard.read_message();
                    let engine_to_gui_message = match message {
                        Ok(msg) => msg,
                        Err(UciReadMessageError::Io(e)) => return Err(e),
                        Err(_) => continue,
                    };

                    if !is_running.load(Ordering::SeqCst) {
                        // Clippy wants this for some reason
                        drop(guard);
                        return Err(io::ErrorKind::ConnectionAborted.into());
                    }

                    match engine_to_gui_message {
                        EngineMessage::Info(info) => {
                            if info_sender.send(info).is_err() {
                                // Return value doesn't matter because the receiver doesn't exist.
                                return Err(io::ErrorKind::Other.into());
                            }
                        }
                        EngineMessage::BestMove(best_move) => return Ok(best_move),
                        _ => (),
                    }
                }
            })?;

        Ok(GuiToEngineUciConnectionGo {
            stop,
            info_receiver,
            thread,
        })
    }

    /// Sends the [`isready`](https://backscattering.de/chess/uci/#gui-isready) message and waits for the [`readyok`](https://backscattering.de/chess/uci/#engine-readyok) response.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading until [`readyok`](https://backscattering.de/chess/uci/#engine-readyok)) errored.
    pub fn is_ready(&mut self) -> io::Result<()> {
        self.send_message(&GuiMessage::IsReady)?;

        loop {
            match self.read_message() {
                Ok(EngineMessage::ReadyOk) => return Ok(()),
                Ok(_) | Err(_) => continue,
            }
        }
    }
}

fn update_id(old_id: &mut Option<IdMessageKind>, new_id: IdMessageKind) {
    let Some(old_id_some) = old_id.take() else {
        *old_id = Some(new_id);
        return;
    };

    *old_id = Some(match (old_id_some, new_id) {
        (IdMessageKind::Author(author), IdMessageKind::Author(_)) => IdMessageKind::Author(author),
        (IdMessageKind::Name(name), IdMessageKind::Name(_)) => IdMessageKind::Name(name),
        (
            IdMessageKind::Author(author),
            IdMessageKind::Name(name) | IdMessageKind::NameAndAuthor { name, .. },
        )
        | (
            IdMessageKind::Name(name),
            IdMessageKind::Author(author) | IdMessageKind::NameAndAuthor { author, .. },
        )
        | (IdMessageKind::NameAndAuthor { name, author }, _) => {
            IdMessageKind::NameAndAuthor { name, author }
        }
    });
}
