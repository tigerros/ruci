use crate::engine::{BestMove, Id, Info};
use crate::errors::{ConnectionError, ReadError, ReadWriteError};
use crate::gui::Go;
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
    /// Creates a new [`Engine`] from an existing process.
    ///
    /// # Errors
    /// [`ConnectionError::Spawn`] is guaranteed not to occur here.
    pub fn from_process(process: &mut Child) -> Result<Self, ConnectionError> {
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
            strict: false,
        })
    }

    #[allow(clippy::missing_errors_doc)]
    /// Creates a new [`Engine`] from the given path.
    pub fn from_path(path: impl AsRef<OsStr>) -> Result<Self, ConnectionError> {
        let mut cmd = Command::new(path);
        let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        #[cfg(windows)]
        // CREATE_NO_WINDOW
        // https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
        let cmd = cmd.creation_flags(0x0800_0000);

        let mut process = cmd.spawn().map_err(ConnectionError::Spawn)?;

        Self::from_process(&mut process)
    }

    /// Sends a message.
    ///
    /// # Errors
    /// See [`AsyncWriteExt::write_all`].
    pub async fn send(&mut self, message: &gui::Message) -> io::Result<()> {
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

    #[allow(clippy::missing_errors_doc)]
    /// Reads a line and attempts to parse it into a [`engine::Message`].
    /// Skips lines which are only composed of whitespace.
    pub async fn read(&mut self) -> Result<engine::Message, ReadError> {
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

    #[allow(clippy::missing_errors_doc)]
    /// Sends the [`Uci`](gui::Uci) message and returns the engine's [`Id`] and a vec
    /// of [`Option`](engine::Option)s once the [`UciOk`](engine::UciOk) message is received.
    pub async fn use_uci(&mut self) -> Result<(Option<Id>, Vec<engine::Option>), ReadWriteError> {
        self.send(&gui::Uci.into())
            .await
            .map_err(ReadWriteError::Write)?;

        let mut options = Vec::with_capacity(40);
        let mut id = None::<Id>;

        loop {
            match self.read().await.map_err(ReadWriteError::Read)? {
                engine::Message::Option(option) => options.push(option),
                engine::Message::Id(new_id) => update_id(&mut id, new_id),
                engine::Message::UciOk(_) => return Ok((id, options)),
                _ => (),
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Sends [`Go`] to the engine and waits for [`BestMove`].
    /// You pass in a function through which [`Info`]s will be sent.
    ///
    /// Note that the engine will only send [`BestMove`]
    /// if you configure the message to set a constraint on the engine's calculation.
    ///
    /// There's examples at the [repo](https://github.com/tigerros/ruci) that shows how you can:
    ///
    /// 1. Start calculating in a separate task.
    /// 2. Save the state to the main task.
    /// 3. Abort the calculation task.
    /// 4. Check the results.
    ///
    /// Or just do it on the same task and wait for completion.
    pub async fn go(
        &mut self,
        message: Go,
        mut info_fn: impl FnMut(Info),
    ) -> Result<BestMove, ReadWriteError> {
        self.send(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            match self.read().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_fn(*info),
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Same as [`Self::go`], but you can pass in an async function.
    pub async fn go_async(
        &mut self,
        message: Go,
        mut info_fn: impl AsyncFnMut(Info),
    ) -> Result<BestMove, ReadWriteError> {
        self.send(&message.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            match self.read().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_fn(*info).await,
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Sends [`IsReady`](gui::IsReady) and waits for [`ReadyOk`](engine::ReadyOk).
    pub async fn is_ready(&mut self) -> Result<(), ReadWriteError> {
        self.send(&gui::IsReady.into())
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            if matches!(
                self.read().await.map_err(ReadWriteError::Read)?,
                engine::Message::ReadyOk(_)
            ) {
                return Ok(());
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
        (Id::Author(_), Id::Author(author)) => Id::Author(author),
        (Id::Name(_), Id::Name(name)) => Id::Name(name),
        (
            Id::NameAndAuthor { .. } | Id::Author(_) | Id::Name(_),
            Id::NameAndAuthor { name, author },
        )
        | (Id::NameAndAuthor { author: _, name } | Id::Name(name), Id::Author(author))
        | (Id::NameAndAuthor { author, name: _ } | Id::Author(author), Id::Name(name)) => {
            Id::NameAndAuthor { name, author }
        }
    });
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::engine::{Id, NormalBestMove};
    use pretty_assertions::{assert_eq, assert_matches};
    use shakmaty::fen::Fen;
    use shakmaty::uci::UciMove;

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

    #[test]
    fn update_id() {
        let mut id = None;

        super::update_id(&mut id, Id::Name("John".to_string()));
        assert_eq!(id, Some(Id::Name("John".to_string())));

        super::update_id(&mut id, Id::Name("Jane".to_string()));
        assert_eq!(id, Some(Id::Name("Jane".to_string())));

        super::update_id(&mut id, Id::Author("Doe".to_string()));
        assert_eq!(
            id,
            Some(Id::NameAndAuthor {
                name: "Jane".to_string(),
                author: "Doe".to_string()
            })
        );

        super::update_id(&mut id, Id::Name("John".to_string()));
        assert_eq!(
            id,
            Some(Id::NameAndAuthor {
                name: "John".to_string(),
                author: "Doe".to_string()
            })
        );
    }

    #[tokio::test]
    async fn is_ready() {
        let mut engine_conn = engine_conn();

        engine_conn.is_ready().await.unwrap();
    }

    #[tokio::test]
    async fn skip_lines() {
        let mut engine_conn = engine_conn();

        engine_conn.send(&gui::Uci.into()).await.unwrap();

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

        engine_conn.send(&gui::Uci.into()).await.unwrap();

        engine_conn
            .send(
                &gui::Position::Fen {
                    moves: Vec::new(),
                    fen: Fen::from_ascii(
                        b"rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
                    )
                    .unwrap(),
                }
                .into(),
            )
            .await
            .unwrap();

        let best_move = engine_conn
            .go(
                Go {
                    depth: Some(5),
                    ..Default::default()
                },
                |_| {},
            )
            .await
            .unwrap();

        assert_eq!(best_move, BestMove::Other);
    }

    #[tokio::test]
    async fn go() {
        let mut engine_conn = engine_conn();

        let best_move = engine_conn
            .go(
                Go {
                    depth: Some(15),
                    ..Default::default()
                },
                |_| {},
            )
            .await
            .unwrap();

        assert_matches!(best_move, BestMove::Normal(_));

        let best_move = best_move.take_normal().unwrap();

        assert_eq!(
            best_move,
            NormalBestMove {
                r#move: UciMove::from_ascii(b"e2e4").unwrap(),
                ponder: Some(UciMove::from_ascii(b"c7c5").unwrap())
            }
        );

        engine_conn.send(&gui::UciNewGame.into()).await.unwrap();
        engine_conn
            .send(
                &gui::Position::StartPos {
                    moves: vec![UciMove::from_ascii(b"d2d4").unwrap()],
                }
                .into(),
            )
            .await
            .unwrap();

        let best_move = engine_conn
            .go_async(
                Go {
                    depth: Some(25),
                    ..Default::default()
                },
                async |_| {},
            )
            .await
            .unwrap();

        assert_matches!(best_move, BestMove::Normal(_));

        let best_move = best_move.take_normal().unwrap();

        assert_eq!(
            best_move,
            NormalBestMove {
                r#move: UciMove::from_ascii(b"g8f6").unwrap(),
                ponder: Some(UciMove::from_ascii(b"c2c4").unwrap())
            }
        );
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
