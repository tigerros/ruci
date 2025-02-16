use crate::auxiliary::{Message, MessageParameterPointer, MessageParseError};
use crate::messages::{BestMove, EngineMessage, Id, Info, Option as OptionMessage};
use crate::messages::{Go, GuiMessage};
use std::marker::PhantomData;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

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
    pub stdout: BufReader<ChildStdout>,
    pub stdin: ChildStdin,
    _phantom: PhantomData<(MSend, MReceive)>,
}

impl<MSend, MReceive> UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    pub const fn new(process: Child, stdout: BufReader<ChildStdout>, stdin: ChildStdin) -> Self {
        Self {
            process,
            stdout,
            stdin,
            _phantom: PhantomData,
        }
    }

    /// # Errors
    ///
    /// [`UciCreationError::Spawn`] is guaranteed not to occur here.
    pub fn from_process(mut process: Child) -> Result<Self, UciCreationError> {
        let Some(stdout) = process.stdout.take() else {
            return Err(UciCreationError::StdoutIsNone);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(UciCreationError::StdinIsNone);
        };

        let stdout = BufReader::new(stdout);

        Ok(Self {
            process,
            stdout,
            stdin,
            _phantom: PhantomData,
        })
    }

    /// Creates a new UCI connection from the given executable path.
    ///
    /// # Errors
    ///
    /// - Spawning the process errored.
    /// - Stdout is [`None`].
    /// - Stdin is [`None`].
    pub fn from_path(path: &str) -> Result<Self, UciCreationError> {
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

        let stdout = BufReader::new(stdout);

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
    /// See [`AsyncWriteExt::write_all`].
    pub async fn send_message(&mut self, message: &MSend) -> io::Result<()> {
        self.stdin.write_all(message.to_string().as_bytes()).await
    }

    /// Skips some lines.
    ///
    /// # Errors
    ///
    /// See [`AsyncBufReadExt::read_line`].
    pub async fn skip_lines(&mut self, count: usize) -> io::Result<()> {
        let mut buf = String::new();

        for _ in 0..count {
            self.stdout.read_line(&mut buf).await?;
        }

        Ok(())
    }

    /// Reads a line and attempts to parse it into a message.
    ///
    /// # Errors
    ///
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    pub async fn read_message(
        &mut self,
    ) -> Result<MReceive, UciReadMessageError<MReceive::ParameterPointer>> {
        let mut line = String::new();
        self.stdout
            .read_line(&mut line)
            .await
            .map_err(UciReadMessageError::Io)?;

        MReceive::from_str(&line).map_err(UciReadMessageError::MessageParse)
    }
}

impl EngineConnection {
    /// Sends the [`GuiMessage::UseUci`] message and returns the engine's ID and a vector of options
    /// once the [`uciok`](https://backscattering.de/chess/uci/#engine-uciok) message is received.
    ///
    /// # Errors
    ///
    /// See [`AsyncWriteExt::write_all`].
    pub async fn use_uci(&mut self) -> io::Result<(Option<Id>, Vec<OptionMessage>)> {
        self.send_message(&GuiMessage::UseUci).await?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<Id>;

        loop {
            let Ok(engine_to_gui_message) = self.read_message().await else {
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
    /// Note that the engine will only send the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove)
    /// message if you set some constraint to prevent the engine from thinking forever.
    /// This can be `depth` or `wtime`/`btime`.
    ///
    /// If you don't have that kind of constraint but want to receive scores and best continuations
    /// (without waiting until the engine finishes with `bestmove`),
    /// use the [`Self::go_async_info`] function.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub async fn go(&mut self, message: Go) -> io::Result<(Vec<Box<Info>>, BestMove)> {
        let message_depth = message.depth;

        self.send_message(&GuiMessage::Go(message)).await?;

        let mut info_messages = Vec::<Box<Info>>::with_capacity(
            message_depth.map_or(100, |depth| depth.saturating_add(3)),
        );

        loop {
            let engine_to_gui_message = match self.read_message().await {
                Ok(msg) => msg,
                Err(UciReadMessageError::Io(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                EngineMessage::Info(info) => info_messages.push(info),
                EngineMessage::BestMove(best_move) => return Ok((info_messages, best_move)),
                _ => (),
            }
        }
    }

    // CLIPPY: Errors doc is in the linked `go` function.
    #[allow(clippy::missing_errors_doc)]
    /// Equivalent to the [`Self::go`] function, but doesn't store a vector of info messages,
    /// and returns only the last one instead.
    pub async fn go_only_last_info(&mut self, message: Go) -> io::Result<(Option<Info>, BestMove)> {
        self.send_message(&GuiMessage::Go(message)).await?;

        let mut last_info_message = None;

        loop {
            let engine_to_gui_message = match self.read_message().await {
                Ok(msg) => msg,
                Err(UciReadMessageError::Io(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                EngineMessage::Info(info) => last_info_message = Some(*info),
                EngineMessage::BestMove(best_move) => return Ok((last_info_message, best_move)),
                _ => (),
            }
        }
    }

    /// Same as [`Self::go`], but instead of returning the [`info`](https://backscattering.de/chess/uci/#engine-info)
    /// messages and the best move together (after waiting for the engine), it immediately returns a tuple which contains:
    ///
    /// - A receiver for the info messages.
    /// - A handle to a task which will send info messages to the receiver and, once the engine returns a
    ///   [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) message, will return that message.
    ///
    /// The spawned task will never panic. It is thus safe to call [`Result::unwrap`] on the handle.
    /// However, the value returned by the handle may be an error, so don't call `unwrap` twice!
    pub fn go_async_info(
        arc_self: Arc<parking_lot::Mutex<Self>>,
        message: Go,
    ) -> (mpsc::Receiver<Box<Info>>, JoinHandle<io::Result<BestMove>>) {
        let (tx, rx) = mpsc::channel(message.depth.map_or(100, |depth| depth.saturating_add(3)));

        (
            rx,
            tokio::spawn(async move {
                let mut channel_closed = false;
                let mut lock = arc_self.lock_arc();

                lock.send_message(&GuiMessage::Go(message)).await?;

                loop {
                    let engine_to_gui_message = match lock.read_message().await {
                        Ok(msg) => msg,
                        Err(UciReadMessageError::Io(e)) => return Err(e),
                        Err(_) => continue,
                    };

                    match engine_to_gui_message {
                        EngineMessage::Info(info) if !channel_closed => {
                            if tx.send(info).await.is_err() {
                                channel_closed = true;
                            }
                        }
                        EngineMessage::BestMove(best_move) => {
                            // Clippy wants this
                            drop(lock);
                            return Ok(best_move);
                        }
                        _ => (),
                    }
                }
            }),
        )
    }

    /// Sends the [`isready`](https://backscattering.de/chess/uci/#gui-isready) message and waits for the [`readyok`](https://backscattering.de/chess/uci/#engine-readyok) response.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading until [`readyok`](https://backscattering.de/chess/uci/#engine-readyok)) errored.
    pub async fn is_ready(&mut self) -> io::Result<()> {
        self.send_message(&GuiMessage::IsReady).await?;

        loop {
            match self.read_message().await {
                Ok(EngineMessage::ReadyOk) => return Ok(()),
                Ok(_) | Err(_) => continue,
            }
        }
    }
}

fn update_id(old_id: &mut Option<Id>, new_id: Id) {
    let Some(old_id_some) = old_id.take() else {
        *old_id = Some(new_id);
        return;
    };

    *old_id = Some(match (old_id_some, new_id) {
        (Id::Author(author), Id::Author(_)) => Id::Author(author),
        (Id::Name(name), Id::Name(_)) => Id::Name(name),
        (Id::Author(author), Id::Name(name) | Id::NameAndAuthor { name, .. })
        | (Id::Name(name), Id::Author(author) | Id::NameAndAuthor { author, .. })
        | (Id::NameAndAuthor { name, author }, _) => Id::NameAndAuthor { name, author },
    });
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn skip_lines() {
        #[allow(clippy::panic)]
        let mut engine_conn = if cfg!(target_os = "windows") {
            EngineConnection::from_path("resources/stockfish-windows-x86-64-avx2.exe").unwrap()
        } else if cfg!(target_os = "linux") {
            EngineConnection::from_path("resources/stockfish-ubuntu-x86-64-avx2").unwrap()
        } else {
            panic!("Unsupported OS");
        };

        engine_conn.send_message(&GuiMessage::UseUci).await.unwrap();

        engine_conn.skip_lines(4).await.unwrap();

        let mut line = String::new();
        engine_conn.stdout.read_line(&mut line).await.unwrap();

        assert_eq!(
            line.trim(),
            "option name Debug Log File type string default <empty>"
        );
    }
}
