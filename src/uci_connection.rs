use crate::messages::{BestMoveMessage, EngineMessage, IdMessageKind, InfoMessage, OptionMessage};
use crate::messages::{GoMessage, GuiMessage};
use crate::{Message, MessageParameterPointer, MessageParseError};
use std::ffi::OsStr;

use std::marker::PhantomData;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio::{io, task};

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
struct UciConnectionInner {
    pub process: Child,
    pub stdout: ChildStdout,
    pub stdin: ChildStdin,
}

#[derive(Debug)]
/// A basis of a UCI connection; use [`EngineConnection`] or [`GuiConnection`] instead.
///
/// This is actually an [`Arc<Mutex<T>>`] wrapper around an inner struct.
/// For that reason, when you want to share a [`UciConnection`] across multiple threads, use [`UciConnection::clone_arc`].
pub struct UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    inner: Arc<Mutex<UciConnectionInner>>,
    _phantom: PhantomData<(MSend, MReceive)>,
}

impl<MSend, MReceive> From<UciConnectionInner> for UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    fn from(inner: UciConnectionInner) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            _phantom: PhantomData,
        }
    }
}

impl<MSend, MReceive> From<Arc<Mutex<UciConnectionInner>>> for UciConnection<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    fn from(inner: Arc<Mutex<UciConnectionInner>>) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
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
    pub fn from_path(path: impl AsRef<OsStr>) -> Result<Self, UciCreationError> {
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
            inner: Arc::new(Mutex::new(UciConnectionInner {
                process,
                stdout,
                stdin,
            })),
            _phantom: PhantomData,
        })
    }

    /// Clones the inner [`Arc<Mutex<T>>`] and puts it in a new [`UciConnection`] instance.
    #[must_use]
    pub fn clone_arc(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            _phantom: PhantomData,
        }
    }

    /// Sends a message.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    pub async fn send_message(&self, message: &MSend) -> io::Result<()> {
        self.inner
            .lock()
            .await
            .stdin
            .write_all(message.to_string().as_bytes())
            .await
    }

    /// Skips some lines.
    ///
    /// # Errors
    ///
    /// See [`Read::read_exact`].
    pub async fn skip_lines(&self, count: usize) -> io::Result<()> {
        let mut buf = [0; 1];
        let mut skipped_count = 0;
        let stdout = &mut self.inner.lock().await.stdout;

        loop {
            stdout.read_exact(&mut buf).await?;

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
    pub async fn read_message(
        &self,
    ) -> Result<MReceive, UciReadMessageError<MReceive::ParameterPointer>> {
        MReceive::from_str(&self.read_line().await.map_err(UciReadMessageError::Io)?)
            .map_err(UciReadMessageError::MessageParse)
    }

    /// Reads one line without the trailing `'\n'` character.
    ///
    /// # Errors
    ///
    /// - Reading resulted in an IO error.
    /// - Parsing the message errors.
    pub async fn read_line(&self) -> io::Result<String> {
        let mut s = String::with_capacity(100);
        let mut buf = [0; 1];
        let stdout = &mut self.inner.lock().await.stdout;

        loop {
            stdout.read_exact(&mut buf).await?;

            if buf[0] == b'\n' {
                break;
            }

            s.push(char::from(buf[0]));
        }

        Ok(s)
    }
}

impl EngineConnection {
    /// Sends the [`GuiMessage::UseUci`] message and returns the engine's ID and a vector of options
    /// once the [`uciok`](https://backscattering.de/chess/uci/#engine-uciok) message is received.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    pub async fn use_uci(&self) -> io::Result<(Option<IdMessageKind>, Vec<OptionMessage>)> {
        self.send_message(&GuiMessage::UseUci).await?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<IdMessageKind>;

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

    /// Sends the [`go`](https://backscattering.de/chess/uci/#gui-go) message to the engine and waits for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) response,
    /// returning it, along with a list of [`info`](https://backscattering.de/chess/uci/#engine-info) messages.
    ///
    /// See also the [`Self::go_task`] function, which spawns a new task and sends back the info messages through a receiver.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub async fn go(&self, message: GoMessage) -> io::Result<(Vec<InfoMessage>, BestMoveMessage)> {
        let mut info_messages = Vec::<InfoMessage>::with_capacity(
            message.depth.map_or(100, |depth| depth.saturating_add(3)),
        );

        self.send_message(&GuiMessage::Go(message)).await?;

        loop {
            let engine_to_gui_message = match self.read_message().await {
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

    /// Spawns a new task which sends the [`go`](https://backscattering.de/chess/uci/#gui-go) message
    /// and waits for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) response.
    /// It sends the [`info`](https://backscattering.de/chess/uci/#engine-info) messages through the returned receiver.
    ///
    /// See also the [`Self::go`] function for a simpler alternative.
    ///
    /// # Errors
    ///
    /// This function does not error, but the task [`JoinHandle`] does.
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub fn go_task(
        &self,
        message: GoMessage,
    ) -> (
        mpsc::Receiver<Box<InfoMessage>>,
        JoinHandle<io::Result<BestMoveMessage>>,
    ) {
        let (info_sender, info_receiver) = mpsc::channel(200);
        let uci = self.clone_arc();

        let handle = task::spawn(async move {
            uci.send_message(&GuiMessage::Go(message)).await?;
            let mut is_info_receiver_open = true;

            loop {
                let engine_to_gui_message = match uci.read_message().await {
                    Ok(msg) => msg,
                    Err(UciReadMessageError::Io(e)) => return Err(e),
                    Err(_) => continue,
                };

                match engine_to_gui_message {
                    EngineMessage::Info(info) => {
                        if is_info_receiver_open && info_sender.send(info).await.is_err() {
                            // Receiver closed, so don't send messages anymore.
                            is_info_receiver_open = false;
                        }
                    }
                    EngineMessage::BestMove(best_move) => return Ok(best_move),
                    _ => (),
                }
            }
        });

        (info_receiver, handle)
    }

    /// Sends the [`isready`](https://backscattering.de/chess/uci/#gui-isready) message and waits for the [`readyok`](https://backscattering.de/chess/uci/#engine-readyok) response.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading until [`readyok`](https://backscattering.de/chess/uci/#engine-readyok)) errored.
    pub async fn is_ready(&self) -> io::Result<()> {
        self.send_message(&GuiMessage::IsReady).await?;

        loop {
            match self.read_message().await {
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
