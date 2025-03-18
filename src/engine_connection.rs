use crate::engine::{BestMove, Info, Registration};
use crate::errors::{ConnectionError, ReadError, ReadWriteError};
use crate::gui::{Go, Register};
use crate::{engine, gui};
use std::ffi::OsStr;
use std::process::Stdio;
use std::str::FromStr;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

/// Communicate with a Chess engine.
#[derive(Debug)]
pub struct Engine {
    pub stdout: BufReader<ChildStdout>,
    pub stdin: ChildStdin,
    /// Whether message parsing errors should be ignored.
    /// This should probably be `false`, because engines do send unrecognized strings.
    ///
    /// If this is `false`, [`ReadError::Parse`] is guaranteed not to occur, including inside
    /// of [`ReadWriteError`]s.
    pub strict: bool,
}

impl Engine {
    /// # Errors
    /// [`ConnectionError::Spawn`] is guaranteed not to occur here.
    pub fn from_process(process: &mut Child, strict: bool) -> Result<Self, ConnectionError> {
        let Some(stdout) = process.stdout.take() else {
            return Err(ConnectionError::StdoutIsNotCaptured);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(ConnectionError::StdinIsNotCaptured);
        };

        let stdout = BufReader::new(stdout);

        Ok(Self {
            stdout,
            stdin,
            strict,
        })
    }

    /// Creates a new connection from the given executable path.
    ///
    /// # Errors
    /// See [`ConnectionError`].
    pub fn from_path(path: impl AsRef<OsStr>, strict: bool) -> Result<Self, ConnectionError> {
        let mut cmd = Command::new(path);
        let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        #[cfg(windows)]
        // CREATE_NO_WINDOW
        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        let cmd = cmd.creation_flags(0x0800_0000);

        let mut process = cmd.spawn().map_err(ConnectionError::Spawn)?;

        Self::from_process(&mut process, strict)
    }

    /// Sends a message.
    ///
    /// # Errors
    /// See [`AsyncWriteExt::write_all`].
    pub async fn send_message(&mut self, message: &gui::Message) -> io::Result<()> {
        self.stdin
            .write_all((message.to_string() + "\n").as_bytes())
            .await
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

    /// Reads a line and attempts to parse it into a [`engine::Message`].
    /// Skips lines which are only composed of whitespace.
    ///
    /// # Errors
    /// See [`ReadError`].
    pub async fn read_message(&mut self) -> Result<engine::Message, ReadError> {
        let mut line = String::new();

        if self.strict {
            loop {
                self.stdout
                    .read_line(&mut line)
                    .await
                    .map_err(ReadError::Io)?;

                if line.is_empty() || line.chars().all(char::is_whitespace) {
                    line.clear();
                    continue;
                }

                break;
            }

            engine::Message::from_str(&line).map_err(ReadError::Parse)
        } else {
            loop {
                self.stdout
                    .read_line(&mut line)
                    .await
                    .map_err(ReadError::Io)?;

                if let Ok(message) = engine::Message::from_str(&line) {
                    return Ok(message);
                }

                line.clear();
            }
        }
    }

    /// Sends the [`Uci`](gui::Uci) message and returns the engine's [`Id`](engine::Id) and a vec
    /// of [`Option`](engine::Option)s once the [`UciOk`](engine::UciOk) message is received.
    ///
    /// # Errors
    /// See [`Self::send_message`].
    pub async fn use_uci(
        &mut self,
    ) -> Result<(Option<engine::Id>, Vec<engine::Option>), ReadWriteError> {
        self.send_message(&gui::Uci.into())
            .await
            .map_err(ReadWriteError::Write)?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<engine::Id>;

        loop {
            match self.read_message().await.map_err(ReadWriteError::Read)? {
                engine::Message::Option(option) => options.push(option),
                engine::Message::Id(new_id) => update_id(&mut id, new_id),
                engine::Message::UciOk(_) => return Ok((id, options)),
                _ => (),
            }
        }
    }

    /// Sends [`Go`] to the engine and waits for [`BestMove`],
    /// returning it, along with a list of [`Info`]s.
    ///
    /// Note that the engine will only send [`BestMove`]
    /// if you configure the message to set a constraint on the engine's calculation.
    ///
    /// See also [`Self::go_only_last_info`], [`Self::go_stream`] and [`Self::go_async_stream`].
    ///
    /// # Errors
    /// See [`Self::send_message`].
    pub async fn go(&mut self, message: Go) -> Result<(Vec<Info>, BestMove), ReadWriteError> {
        let message_depth = message.depth;

        self.send_message(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        let mut info_messages =
            Vec::<Info>::with_capacity(message_depth.map_or(100, |depth| depth.saturating_mul(5)));

        loop {
            match self.read_message().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_messages.push(*info),
                engine::Message::BestMove(best_move) => return Ok((info_messages, best_move)),
                _ => (),
            }
        }
    }

    /// Same as [`Self::go`], but instead of storing a vec of [`Info`]s,
    /// returns just the last one.
    ///
    /// # Errors
    /// See [`Self::go`].
    pub async fn go_only_last_info(
        &mut self,
        message: Go,
    ) -> Result<(Option<Info>, BestMove), ReadWriteError> {
        self.send_message(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        let mut last_info_message = None;

        loop {
            match self.read_message().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => last_info_message = Some(*info),
                engine::Message::BestMove(best_move) => return Ok((last_info_message, best_move)),
                _ => (),
            }
        }
    }

    /// Same as [`Self::go`], but instead of storing a vec of [`Info`]s,
    /// you pass in a function that is called every time an [`Info`] is encountered.
    ///
    /// This allows you to receive [`Info`]s before the engine is done calculating.
    /// There's an example at the [repo](https://github.com/tigerros/ruci) that shows how you can:
    ///
    /// 1. Start calculating in a separate task.
    /// 2. Save the state to the main task.
    /// 3. Abort the calculation task.
    /// 4. Check the results.
    ///
    /// # Errors
    /// See [`Self::go`].
    pub async fn go_stream(
        &mut self,
        message: Go,
        mut info_fn: impl FnMut(Info),
    ) -> Result<BestMove, ReadWriteError> {
        self.send_message(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            match self.read_message().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_fn(*info),
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    /// Same as [`Self::go_stream`], but you can pass in an async function.
    ///
    /// # Errors
    /// See [`Self::go`].
    pub async fn go_async_stream(
        &mut self,
        message: Go,
        mut info_fn: impl AsyncFnMut(Info),
    ) -> Result<BestMove, ReadWriteError> {
        self.send_message(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            match self.read_message().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_fn(*info).await,
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    /// Sends [`IsReady`](gui::IsReady) and waits for [`ReadyOk`](engine::ReadyOk).
    ///
    /// # Errors
    /// See [`Self::send_message`].
    pub async fn is_ready(&mut self) -> Result<(), ReadWriteError> {
        self.send_message(&gui::IsReady.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            if matches!(
                self.read_message().await.map_err(ReadWriteError::Read)?,
                engine::Message::ReadyOk(_)
            ) {
                return Ok(());
            }
        }
    }

    /// Sends [`Register`] and waits for [`Registration`].
    ///
    /// # Errors
    /// See [`Self::send_message`].
    pub async fn register(&mut self, register: Register) -> Result<Registration, ReadWriteError> {
        self.send_message(&register.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            if let engine::Message::Registration(registration) =
                self.read_message().await.map_err(ReadWriteError::Read)?
            {
                return Ok(registration);
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
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const ENGINE_EXE: &str = if cfg!(target_os = "windows") {
        "resources/stockfish-windows-x86-64-avx2.exe"
    } else if cfg!(target_os = "linux") {
        "resources/stockfish-ubuntu-x86-64-avx2"
    } else {
        panic!("Unsupported OS");
    };

    fn engine_conn() -> Engine {
        Engine::from_path(ENGINE_EXE, false).unwrap()
    }

    #[tokio::test]
    async fn is_ready() {
        let mut engine_conn = engine_conn();

        engine_conn.is_ready().await.unwrap();
    }

    #[tokio::test]
    async fn skip_lines() {
        let mut engine_conn = engine_conn();

        engine_conn.send_message(&gui::Uci.into()).await.unwrap();

        engine_conn.skip_lines(4).await.unwrap();

        let mut line = String::new();
        engine_conn.stdout.read_line(&mut line).await.unwrap();

        assert_eq!(
            line.trim(),
            "option name Debug Log File type string default <empty>"
        );
    }

    /// See the [`BestMove::Other`](BestMove::Other) docs for what this tests.
    #[tokio::test]
    async fn analyze_checkmate() {
        let mut engine_conn = engine_conn();

        engine_conn.send_message(&gui::Uci.into()).await.unwrap();

        engine_conn
            .send_message(
                &gui::Position::Fen {
                    moves: None,
                    fen: "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3"
                        .to_string(),
                }
                .into(),
            )
            .await
            .unwrap();

        let (_, best_move) = engine_conn
            .go_only_last_info(Go {
                depth: Some(5),
                ..Default::default()
            })
            .await
            .unwrap();

        assert_eq!(best_move, BestMove::Other);
    }

    #[allow(clippy::too_many_lines)]
    #[tokio::test]
    async fn use_uci() {
        use engine::{Id, Option};

        let mut engine_conn = engine_conn();

        let (id, options) = engine_conn.use_uci().await.unwrap();

        let mut options_str = String::new();

        for option in &options {
            options_str.push_str(&format!("{option}\n"));
        }

        assert_eq!(
            options_str.trim(),
            r"option name Debug Log File type string default <empty>
option name NumaPolicy type string default auto
option name Threads type spin default 1 min 1 max 1024
option name Hash type spin default 16 min 1 max 33554432
option name Clear Hash type button
option name Ponder type check default false
option name MultiPV type spin default 1 min 1 max 256
option name Skill Level type spin default 20 min 0 max 20
option name Move Overhead type spin default 10 min 0 max 5000
option name nodestime type spin default 0 min 0 max 10000
option name UCI_Chess960 type check default false
option name UCI_LimitStrength type check default false
option name UCI_Elo type spin default 1320 min 1320 max 3190
option name UCI_ShowWDL type check default false
option name SyzygyPath type string default <empty>
option name SyzygyProbeDepth type spin default 1 min 1 max 100
option name Syzygy50MoveRule type check default true
option name SyzygyProbeLimit type spin default 7 min 0 max 7
option name EvalFile type string default nn-1111cefa1111.nnue
option name EvalFileSmall type string default nn-37f18f62d772.nnue"
        );

        assert_eq!(
            id,
            Some(Id::NameAndAuthor {
                name: "Stockfish 17".to_string(),
                author: "the Stockfish developers (see AUTHORS file)".to_string()
            })
        );

        let mut options = options.into_iter();

        assert_eq!(
            options.next(),
            Some(Option::String {
                name: "Debug Log File".to_string(),
                default: Some("<empty>".to_string())
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::String {
                name: "NumaPolicy".to_string(),
                default: Some("auto".to_string())
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "Threads".to_string(),
                default: Some(1),
                min: Some(1),
                max: Some(1024)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "Hash".to_string(),
                default: Some(16),
                min: Some(1),
                max: Some(33_554_432)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Button {
                name: "Clear Hash".to_string(),
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Check {
                name: "Ponder".to_string(),
                default: Some(false)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "MultiPV".to_string(),
                default: Some(1),
                min: Some(1),
                max: Some(256)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "Skill Level".to_string(),
                default: Some(20),
                min: Some(0),
                max: Some(20)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "Move Overhead".to_string(),
                default: Some(10),
                min: Some(0),
                max: Some(5000)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "nodestime".to_string(),
                default: Some(0),
                min: Some(0),
                max: Some(10000)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Check {
                name: "UCI_Chess960".to_string(),
                default: Some(false)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Check {
                name: "UCI_LimitStrength".to_string(),
                default: Some(false)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "UCI_Elo".to_string(),
                default: Some(1320),
                min: Some(1320),
                max: Some(3190)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Check {
                name: "UCI_ShowWDL".to_string(),
                default: Some(false)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::String {
                name: "SyzygyPath".to_string(),
                default: Some("<empty>".to_string())
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "SyzygyProbeDepth".to_string(),
                default: Some(1),
                min: Some(1),
                max: Some(100)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Check {
                name: "Syzygy50MoveRule".to_string(),
                default: Some(true)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::Spin {
                name: "SyzygyProbeLimit".to_string(),
                default: Some(7),
                min: Some(0),
                max: Some(7)
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::String {
                name: "EvalFile".to_string(),
                default: Some("nn-1111cefa1111.nnue".to_string())
            })
        );

        assert_eq!(
            options.next(),
            Some(Option::String {
                name: "EvalFileSmall".to_string(),
                default: Some("nn-37f18f62d772.nnue".to_string())
            })
        );
    }
}
