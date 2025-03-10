use crate::{engine, errors, gui, Message};
use std::ffi::OsStr;
use std::process::Stdio;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Allows you to more easily communicate with the engine.
///
/// There are some convenience methods for specific messages, but it's not comprehensive.
#[derive(Debug)]
pub struct EngineConnection {
    pub process: Child,
    pub stdout: BufReader<ChildStdout>,
    pub stdin: ChildStdin,
}

impl EngineConnection {
    /// # Errors
    /// [`errors::ConnectionError::Spawn`] is guaranteed not to occur here.
    pub fn from_process(mut process: Child) -> Result<Self, errors::ConnectionError> {
        let Some(stdout) = process.stdout.take() else {
            return Err(errors::ConnectionError::StdoutIsNotCaptured);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(errors::ConnectionError::StdinIsNotCaptured);
        };

        let stdout = BufReader::new(stdout);

        Ok(Self {
            process,
            stdout,
            stdin,
        })
    }

    /// Creates a new connection from the given executable path.
    ///
    /// # Errors
    /// See [`errors::ConnectionError`].
    pub fn from_path(path: impl AsRef<OsStr>) -> Result<Self, errors::ConnectionError> {
        let mut cmd = Command::new(path);
        let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        #[cfg(windows)]
        // CREATE_NO_WINDOW
        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        let cmd = cmd.creation_flags(0x0800_0000);

        let mut process = cmd.spawn().map_err(errors::ConnectionError::Spawn)?;

        let Some(stdout) = process.stdout.take() else {
            return Err(errors::ConnectionError::StdoutIsNotCaptured);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(errors::ConnectionError::StdinIsNotCaptured);
        };

        let stdout = BufReader::new(stdout);

        Ok(Self {
            process,
            stdout,
            stdin,
        })
    }

    /// Sends a message.
    ///
    /// # Errors
    /// See [`AsyncWriteExt::write_all`].
    pub async fn send_message(&mut self, message: &gui::Message) -> io::Result<()> {
        self.stdin.write_all(message.to_string().as_bytes()).await
    }

    /// Skips some lines.
    ///
    /// # Errors
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
    /// See [`errors::ReadMessageError`].
    pub async fn read_message(&mut self) -> Result<engine::Message, errors::ReadMessageError> {
        let mut line = String::new();
        self.stdout
            .read_line(&mut line)
            .await
            .map_err(errors::ReadMessageError::Io)?;

        if let Message::Engine(engine_message) =
            Message::from_str(&line).map_err(errors::ReadMessageError::MessageParse)?
        {
            Ok(engine_message)
        } else {
            Err(errors::ReadMessageError::GotGuiMessage)
        }
    }

    /// Sends the [`uci`](https://backscattering.de/chess/uci/#gui-uci) message and returns the engine's ID and a vector of options
    /// once the [`uciok`](https://backscattering.de/chess/uci/#engine-uciok) message is received.
    ///
    /// # Errors
    /// See [`AsyncWriteExt::write_all`].
    pub async fn use_uci(&mut self) -> io::Result<(Option<engine::Id>, Vec<engine::Option>)> {
        self.send_message(&gui::Message::UseUci).await?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<engine::Id>;

        loop {
            let Ok(engine_to_gui_message) = self.read_message().await else {
                continue;
            };

            match engine_to_gui_message {
                engine::Message::Option(option) => options.push(option),
                engine::Message::Id(new_id) => update_id(&mut id, new_id),
                engine::Message::UciOk => return Ok((id, options)),
                _ => (),
            }
        }
    }

    /// Sends the [`go`](https://backscattering.de/chess/uci/#gui-go) message to the engine and waits for the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) message response,
    /// returning it, along with a list of [`info`](https://backscattering.de/chess/uci/#engine-info) messages.
    ///
    /// Note that the engine will only send the `bestmove`
    /// message if you set some constraint to prevent the engine from thinking forever.
    /// This can be [`depth`](https://backscattering.de/chess/uci/#engine-info-depth) or [`wtime`](https://backscattering.de/chess/uci/#gui-go-wtime)/[`btime`](https://backscattering.de/chess/uci/#gui-go-btime).
    ///
    /// If you don't have that kind of constraint but want to receive scores and best continuations
    /// (without waiting until the engine finishes with `bestmove`),
    /// use the [`Self::go_async_info`] function.
    ///
    /// # Errors
    /// - Writing (sending the message) errored.
    /// - Reading (reading back the responses) errored.
    pub async fn go(
        &mut self,
        message: gui::Go,
    ) -> io::Result<(Vec<Box<engine::Info>>, engine::BestMove)> {
        let message_depth = message.depth;

        self.send_message(&message.into()).await?;

        let mut info_messages = Vec::<Box<engine::Info>>::with_capacity(
            message_depth.map_or(100, |depth| depth.saturating_add(3)),
        );

        loop {
            let engine_to_gui_message = match self.read_message().await {
                Ok(msg) => msg,
                Err(errors::ReadMessageError::Io(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                engine::Message::Info(info) => info_messages.push(info),
                engine::Message::BestMove(best_move) => return Ok((info_messages, best_move)),
                _ => (),
            }
        }
    }

    // CLIPPY: Errors doc is in the linked `go` function.
    #[allow(clippy::missing_errors_doc)]
    /// Same as [`Self::go`], but doesn't store a vector of [`info`](https://backscattering.de/chess/uci/#engine-info)
    /// messages, instead only returning the last one.
    pub async fn go_only_last_info(
        &mut self,
        message: gui::Go,
    ) -> io::Result<(Option<engine::Info>, engine::BestMove)> {
        self.send_message(&message.into()).await?;

        let mut last_info_message = None;

        loop {
            let engine_to_gui_message = match self.read_message().await {
                Ok(msg) => msg,
                Err(errors::ReadMessageError::Io(e)) => return Err(e),
                Err(_) => continue,
            };

            match engine_to_gui_message {
                engine::Message::Info(info) => last_info_message = Some(*info),
                engine::Message::BestMove(best_move) => return Ok((last_info_message, best_move)),
                _ => (),
            }
        }
    }

    /// Same as [`Self::go`], but instead of returning the [`info`](https://backscattering.de/chess/uci/#engine-info)
    /// messages and the [`bestmove`](https://backscattering.de/chess/uci/#engine-bestmove) together (after waiting for the engine), it immediately returns a tuple which contains:
    ///
    /// - A receiver for the `info` messages.
    /// - A handle to a task which will send `info` messages to the receiver and, once the engine returns a
    ///   `bestmove` message, will return that message.
    ///
    /// The spawned task will never panic. It is thus safe to call [`Result::unwrap`] on the handle.
    /// However, the value returned by the handle may be an error, so don't call `unwrap` twice!
    pub fn go_async_info(
        arc_self: Arc<parking_lot::Mutex<Self>>,
        message: gui::Go,
    ) -> (
        mpsc::Receiver<Box<engine::Info>>,
        JoinHandle<io::Result<engine::BestMove>>,
    ) {
        let (tx, rx) = mpsc::channel(message.depth.map_or(100, |depth| depth.saturating_add(3)));

        (
            rx,
            tokio::spawn(async move {
                let mut channel_closed = false;
                let mut lock = arc_self.lock_arc();

                lock.send_message(&message.into()).await?;

                loop {
                    let engine_to_gui_message = match lock.read_message().await {
                        Ok(msg) => msg,
                        Err(errors::ReadMessageError::Io(e)) => return Err(e),
                        Err(_) => continue,
                    };

                    match engine_to_gui_message {
                        engine::Message::Info(info) if !channel_closed => {
                            if tx.send(info).await.is_err() {
                                channel_closed = true;
                            }
                        }
                        engine::Message::BestMove(best_move) => {
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
    /// - Writing (sending the message) errored.
    /// - Reading (reading until `readyok`) errored.
    pub async fn is_ready(&mut self) -> io::Result<()> {
        self.send_message(&gui::Message::IsReady).await?;

        loop {
            match self.read_message().await {
                Ok(engine::Message::ReadyOk) => return Ok(()),
                Ok(_) | Err(_) => continue,
            }
        }
    }
}

fn update_id(old_id: &mut Option<engine::Id>, new_id: engine::Id) {
    let Some(old_id_some) = old_id.take() else {
        *old_id = Some(new_id);
        return;
    };

    *old_id = Some(match (old_id_some, new_id) {
        (engine::Id::Author(author), engine::Id::Author(_)) => engine::Id::Author(author),
        (engine::Id::Name(name), engine::Id::Name(_)) => engine::Id::Name(name),
        (
            engine::Id::Author(author),
            engine::Id::Name(name) | engine::Id::NameAndAuthor { name, .. },
        )
        | (
            engine::Id::Name(name),
            engine::Id::Author(author) | engine::Id::NameAndAuthor { author, .. },
        )
        | (engine::Id::NameAndAuthor { name, author }, _) => {
            engine::Id::NameAndAuthor { name, author }
        }
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

        engine_conn
            .send_message(&gui::Message::UseUci)
            .await
            .unwrap();

        engine_conn.skip_lines(4).await.unwrap();

        let mut line = String::new();
        engine_conn.stdout.read_line(&mut line).await.unwrap();

        assert_eq!(
            line.trim(),
            "option name Debug Log File type string default <empty>"
        );
    }
}
