use crate::messages::engine_to_gui::{
    BestMoveMessage, EngineToGuiMessage, IdMessageKind, InfoMessage, OptionMessage,
};
use crate::messages::gui_to_engine::{GoMessage, GuiToEngineMessage};
use crate::{Message, MessageParameterPointer, MessageParseError};
use shakmaty::uci::Uci;
use std::io;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::thread::{JoinHandle, Thread};

pub type GuiToEngineUciConnection = UciConnection<GuiToEngineMessage, EngineToGuiMessage>;
pub type EngineToGuiUciConnection = UciConnection<EngineToGuiMessage, GuiToEngineMessage>;
pub type GuiToEngineUciConnectionSync = UciConnectionSync<GuiToEngineMessage, EngineToGuiMessage>;
pub type EngineToGuiUciConnectionSync = UciConnectionSync<EngineToGuiMessage, GuiToEngineMessage>;

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
        let mut cmd = Command::new(path);
        let mut cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            // CREATE_NO_WINDOW
            // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
            cmd = cmd.creation_flags(0x08000000);
        }

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

    /// Skips a number of lines.
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
    pub fn read_message(
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

pub struct UciConnectionSync<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    pub inner: Arc<Mutex<UciConnection<MSend, MReceive>>>,
    is_thread_running: Arc<AtomicBool>,
}

#[derive(Debug)]
pub enum AbortThreadError<'a, MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    PoisonError(PoisonError<MutexGuard<'a, UciConnection<MSend, MReceive>>>),
    Io(io::Error),
}

impl<MSend, MReceive> UciConnectionSync<MSend, MReceive>
where
    MSend: Message,
    MReceive: Message,
{
    pub fn new(inner: UciConnection<MSend, MReceive>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
            is_thread_running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn is_thread_running(&self) -> bool {
        self.is_thread_running.load(Ordering::SeqCst)
    }
}

impl EngineToGuiUciConnectionSync {
    pub fn abort_thread(&self) {
        self.is_thread_running.store(false, Ordering::SeqCst);
    }
}

impl GuiToEngineUciConnectionSync {
    pub fn abort_thread(
        &self,
    ) -> Result<(), AbortThreadError<'_, GuiToEngineMessage, EngineToGuiMessage>> {
        self.is_thread_running.store(false, Ordering::SeqCst);
        self.inner
            .lock()
            .map_err(AbortThreadError::PoisonError)?
            .send_message(&GuiToEngineMessage::Stop)
            .map_err(AbortThreadError::Io)?;

        Ok(())
    }

    /// Sends the [`GuiToEngineMessage::UseUci`] message and returns the engine's ID and a vector of options
    /// once the `uciok` message is received.
    ///
    /// # Errors
    ///
    /// See [`Write::write_all`].
    pub fn use_uci(
        &mut self,
        option_sender: Sender<OptionMessage>,
    ) -> JoinHandle<io::Result<Option<IdMessageKind>>> {
        let inner = self.inner.clone();
        let is_thread_running = self.is_thread_running.clone();

        thread::spawn(move || {
            if !is_thread_running.load(Ordering::SeqCst) {
                return Err(io::ErrorKind::ConnectionAborted.into());
            }

            #[allow(clippy::unwrap_used)]
            let mut inner = inner.lock().unwrap();

            inner.send_message(&GuiToEngineMessage::UseUci)?;

            let mut id = None::<IdMessageKind>;

            loop {
                let Ok(engine_to_gui_message) = inner.read_message() else {
                    continue;
                };

                match engine_to_gui_message {
                    EngineToGuiMessage::Option(option) => {
                        if option_sender.send(option).is_err() {
                            // Return value doesn't matter because the receiver doesn't exist.
                            return Err(io::ErrorKind::Other.into());
                        }
                    }
                    EngineToGuiMessage::Id(new_id) => update_id(&mut id, new_id),
                    EngineToGuiMessage::UciOk => return Ok(id),
                    _ => (),
                }
            }
        })
    }

    /// Sends the `go` message to the engine and waits for the `bestmove` message response.
    /// Sends back `info` messages through the given sender.
    ///
    /// # Errors
    ///
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub fn go(
        &mut self,
        message: GoMessage,
        info_sender: Sender<Box<InfoMessage>>,
    ) -> JoinHandle<io::Result<BestMoveMessage>> {
        let inner = self.inner.clone();
        let is_thread_running = self.is_thread_running.clone();

        thread::spawn(move || {
            if !is_thread_running.load(Ordering::SeqCst) {
                return Err(io::ErrorKind::ConnectionAborted.into());
            }

            #[allow(clippy::unwrap_used)]
            let mut inner = inner.lock().unwrap();
            inner.send_message(&GuiToEngineMessage::Go(message))?;

            loop {
                if !is_thread_running.load(Ordering::SeqCst) {
                    return Err(io::ErrorKind::ConnectionAborted.into());
                }

                let message = inner.read_message();
                if !is_thread_running.load(Ordering::SeqCst) {
                    return Err(io::ErrorKind::ConnectionAborted.into());
                }
                let engine_to_gui_message = match message {
                    Ok(msg) => msg,
                    Err(UciReadMessageError::Read(e)) => return Err(e),
                    Err(_) => continue,
                };

                match engine_to_gui_message {
                    EngineToGuiMessage::Info(info) => {
                        if info_sender.send(info).is_err() {
                            // Return value doesn't matter because the receiver doesn't exist.
                            return Err(io::ErrorKind::Other.into());
                        }
                    }
                    EngineToGuiMessage::BestMove(best_move) => return Ok(best_move),
                    _ => (),
                }
            }
        })
    }
    //
    // /// Sends the `isready` message and waits for the `readyok` response.
    // ///
    // /// # Errors
    // ///
    // /// - Writing (sending the message) errored.
    // /// - Reading (reading until `readyok`) errored.
    // pub fn is_ready(&mut self) -> io::Result<()> {
    //     self.send_message(&GuiToEngineMessage::IsReady)?;
    //
    //     loop {
    //         match self.read_message() {
    //             Ok(EngineToGuiMessage::ReadyOk) => return Ok(()),
    //             Ok(_) | Err(_) => continue,
    //         }
    //     }
    // }
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
            IdMessageKind::Name(name) | IdMessageKind::NameAndAuthor(name, _),
        )
        | (
            IdMessageKind::Name(name),
            IdMessageKind::Author(author) | IdMessageKind::NameAndAuthor(_, author),
        )
        | (IdMessageKind::NameAndAuthor(name, author), _) => {
            IdMessageKind::NameAndAuthor(name, author)
        }
    });
}
