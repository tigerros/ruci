// TODO: change docs of async functions to just link to the original function
use crate::errors::{ReadError, ReadWriteError};
use crate::Engine;
#[cfg(feature = "tokio-process")]
use crate::FromProcessError;
use crate::{engine, gui};
use crate::{BestMove, Id, Info};
use crate::{Go, MessageParseError};
use std::str::FromStr;
use tokio::io;
#[cfg(feature = "tokio-process")]
use tokio::io::BufReader;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
#[cfg(feature = "tokio-process")]
use tokio::process::{Child, ChildStdin, ChildStdout};

#[cfg(feature = "tokio-process")]
impl Engine<BufReader<ChildStdout>, ChildStdin> {
    #[allow(clippy::missing_errors_doc)]
    /// Uses the `stdin` and `stdout` from an existing process.
    ///
    /// See also [`Engine.strict`](Engine#structfield.strict).
    pub fn from_process_async(process: &mut Child, strict: bool) -> Result<Self, FromProcessError> {
        let Some(stdout) = process.stdout.take() else {
            return Err(FromProcessError::StdoutNotCaptured);
        };

        let Some(stdin) = process.stdin.take() else {
            return Err(FromProcessError::StdinNotCaptured);
        };

        let stdout = BufReader::new(stdout);

        Ok(Self {
            engine: stdout,
            gui: stdin,
            strict,
        })
    }
}

impl<E, G> Engine<E, G>
where
    E: AsyncBufRead + Unpin,
    G: AsyncWrite + Unpin,
{
    // CLIPPY: Message is implemented for borrows as well
    #[allow(clippy::needless_pass_by_value)]
    /// Sends a message.
    ///
    /// # Errors
    /// See [`AsyncWriteExt::write_all`].
    pub async fn send_async<M>(&mut self, message: M) -> io::Result<()>
    where
        M: gui::traits::Message,
    {
        self.gui
            .write_all((message.to_string() + "\n").as_bytes())
            .await
    }

    /// Skips some lines.
    ///
    /// # Errors
    /// See [`AsyncBufReadExt::read_until`].
    pub async fn skip_lines_async(&mut self, count: usize) -> io::Result<()> {
        let mut buf = Vec::with_capacity(512);

        for _ in 0..count {
            let bytes = self.engine.read_until(b'\n', &mut buf).await?;

            if bytes == 0 {
                break;
            }

            buf.clear();
        }

        Ok(())
    }

    #[allow(clippy::missing_errors_doc)]
    /// Reads a line and attempts to parse it into a [`engine::Message`].
    /// Skips lines which are only composed of whitespace.
    pub async fn read_async<M>(&mut self) -> Result<M, ReadError>
    where
        M: crate::traits::Message + FromStr<Err = MessageParseError>,
    {
        let mut line = String::new();

        if self.strict {
            loop {
                self.engine
                    .read_line(&mut line)
                    .await
                    .map_err(ReadError::Io)?;

                if line.is_empty() || line.chars().all(char::is_whitespace) {
                    line.clear();
                    continue;
                }

                break;
            }

            M::from_str(&line).map_err(ReadError::Parse)
        } else {
            loop {
                self.engine
                    .read_line(&mut line)
                    .await
                    .map_err(ReadError::Io)?;

                if let Ok(message) = M::from_str(&line) {
                    return Ok(message);
                }

                line.clear();
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Sends the [`Uci`](gui::Uci) message and returns the engine's [`Id`]
    /// once the [`UciOk`](engine::UciOk) message is received.
    ///
    /// When an [`Option`](engine::Option) is encountered, the `option_receiver` function is called.
    pub async fn use_uci_async<'m>(
        &mut self,
        mut option_receiver: impl AsyncFnMut(crate::Option<'m>),
    ) -> Result<Option<Id<'m>>, ReadWriteError> {
        self.send_async(crate::Uci)
            .await
            .map_err(ReadWriteError::Write)?;

        let mut id = None::<Id>;

        loop {
            match self.read_async().await.map_err(ReadWriteError::Read)? {
                engine::Message::Option(option) => option_receiver(option).await,
                engine::Message::Id(new_id) => {
                    id = Some(if let Some(id) = id {
                        id.updated(new_id)
                    } else {
                        new_id
                    });
                }
                engine::Message::UciOk(_) => return Ok(id),
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
    /// There's examples at the [repo](https://github.com/tigerros/ruci) that show this
    /// function being used concurrently.
    pub async fn go_async<'m>(
        &mut self,
        message: &Go<'_>,
        mut info_receiver: impl AsyncFnMut(Info<'m>),
    ) -> Result<BestMove, ReadWriteError> {
        self.send_async(message)
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            match self.read_async().await.map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_receiver(*info).await,
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Sends [`IsReady`](gui::IsReady) and waits for [`ReadyOk`](engine::ReadyOk).
    pub async fn is_ready_async(&mut self) -> Result<(), ReadWriteError> {
        self.send_async(crate::IsReady)
            .await
            .map_err(ReadWriteError::Write)?;

        loop {
            if let engine::Message::ReadyOk(_) =
                self.read_async().await.map_err(ReadWriteError::Read)?
            {
                return Ok(());
            }
        }
    }
}

#[cfg(all(test, feature = "tokio-process"))]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::Position;
    use crate::{NormalBestMove, OptionType};
    use alloc::borrow::Cow;
    use pretty_assertions::{assert_eq, assert_matches};
    use shakmaty::fen::Fen;
    use shakmaty::uci::UciMove;
    use std::process::Stdio;
    use tokio::io::BufReader;
    use tokio::process::{ChildStdin, ChildStdout, Command};

    const ENGINE_EXE: &str = if cfg!(target_os = "windows") {
        "resources/stockfish-windows-x86-64-avx2.exe"
    } else if cfg!(target_os = "linux") {
        "resources/stockfish-ubuntu-x86-64-avx2"
    } else {
        panic!("Unsupported OS");
    };

    /// Use the second variable in the tuple to wait on the process.
    fn engine() -> (
        Engine<BufReader<ChildStdout>, ChildStdin>,
        impl AsyncFnMut(),
    ) {
        let mut cmd = Command::new(ENGINE_EXE);
        let cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
        let mut process = cmd.spawn().unwrap();

        (
            Engine::<BufReader<ChildStdout>, ChildStdin>::from_process_async(&mut process, false)
                .unwrap(),
            async move || {
                process.kill().await.unwrap();
                process.wait().await.unwrap();
            },
        )
    }

    #[tokio::test]
    async fn is_ready() {
        let (mut engine, mut wait) = engine();

        engine.is_ready_async().await.unwrap();
        wait().await;
    }

    // CLIPPY: It's literally used???
    #[allow(clippy::extra_unused_lifetimes)]
    #[tokio::test]
    async fn _lifetimes<'a>() {
        let mut engine = Engine {
            engine: b"uciok\noption name n type button".as_slice(),
            gui: Vec::new(),
            strict: false,
        };

        let _: engine::Message<'static> = engine.read_async::<engine::Message>().await.unwrap();

        assert_eq!(
            engine.read_async::<engine::Message>().await.unwrap(),
            engine::Message::Option(crate::Option {
                name: Cow::Borrowed::<'a>("n"),
                r#type: OptionType::Button,
            })
        );
    }

    #[tokio::test]
    async fn strict() {
        let (mut engine, mut wait) = engine();

        engine.strict = true;

        // Stockfish sends an unrecognized string at the very beginning
        assert!(matches!(
            engine.use_uci_async(async |_| {}).await,
            Err(ReadWriteError::Read(ReadError::Parse(
                MessageParseError::NoMessage {
                    expected: "engine UCI message"
                }
            )))
        ));

        let mut engine = Engine {
            engine: &mut b"id name Big Tuna author Fischer\n\n\n\toption   name Horsey range type string default the biggest!!\nuciok".as_slice(),
            gui: Vec::new(),
            strict: true,
        };

        engine
            .use_uci_async(async |option| {
                assert_eq!(
                    option,
                    crate::Option {
                        name: Cow::Borrowed("Horsey range"),
                        r#type: OptionType::String {
                            default: Some(Cow::Borrowed("the biggest!!"))
                        }
                    }
                );
            })
            .await
            .unwrap();

        assert_eq!(engine.gui, b"uci\n");
        wait().await;
    }

    #[tokio::test]
    async fn skip_lines() {
        let (mut engine, mut wait) = engine();

        engine.send_async(crate::Uci).await.unwrap();

        engine.skip_lines_async(4).await.unwrap();

        let mut line = String::new();
        engine.engine.read_line(&mut line).await.unwrap();

        assert_eq!(
            line.trim(),
            "option name Debug Log File type string default <empty>"
        );
        wait().await;
    }

    /// See the [`BestMove::Other`](BestMove::Other) docs for what this tests.
    #[tokio::test]
    async fn analyze_checkmate() {
        let (mut engine, mut wait) = engine();

        engine.send_async(crate::Uci).await.unwrap();

        engine
            .send_async(Position::Fen {
                moves: Cow::Borrowed(&[]),
                fen: Cow::Owned(
                    Fen::from_ascii(
                        b"rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
                    )
                    .unwrap(),
                ),
            })
            .await
            .unwrap();

        let best_move = engine
            .go_async(
                &Go {
                    depth: Some(5),
                    ..Default::default()
                },
                async |_| {},
            )
            .await
            .unwrap();

        assert_eq!(best_move, BestMove::Other);
        wait().await;
    }

    #[tokio::test]
    async fn go() {
        let (mut engine, mut wait) = engine();

        let best_move = engine
            .go_async(
                &Go {
                    depth: Some(15),
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
                r#move: UciMove::from_ascii(b"e2e4").unwrap(),
                ponder: Some(UciMove::from_ascii(b"c7c5").unwrap())
            }
        );

        engine.send_async(crate::UciNewGame).await.unwrap();
        engine
            .send_async(Position::StartPos {
                moves: Cow::Borrowed(&[UciMove::from_ascii(b"d2d4").unwrap()]),
            })
            .await
            .unwrap();

        let best_move = engine
            .go_async(
                &Go {
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
        wait().await;
    }

    #[allow(clippy::too_many_lines)]
    #[tokio::test]
    async fn use_uci() {
        use crate::{Id, Option};
        use core::fmt::Write;

        let (mut engine, mut wait) = engine();

        let mut options = Vec::new();
        let id = engine
            .use_uci_async(async |option| options.push(option))
            .await
            .unwrap();

        let mut options_str = String::new();

        for option in &options {
            let _ = writeln!(options_str, "{option}");
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
                name: "Stockfish 17".into(),
                author: "the Stockfish developers (see AUTHORS file)".into()
            })
        );

        let mut options = options.into_iter();

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Debug Log File"),
                r#type: OptionType::String {
                    default: Some(Cow::Borrowed("<empty>"))
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("NumaPolicy"),
                r#type: OptionType::String {
                    default: Some(Cow::Borrowed("auto"))
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Threads"),
                r#type: OptionType::Spin {
                    default: Some(1),
                    min: Some(1),
                    max: Some(1024)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Hash"),
                r#type: OptionType::Spin {
                    default: Some(16),
                    min: Some(1),
                    max: Some(33_554_432)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Clear Hash"),
                r#type: OptionType::Button
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Ponder"),
                r#type: OptionType::Check {
                    default: Some(false)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("MultiPV"),
                r#type: OptionType::Spin {
                    default: Some(1),
                    min: Some(1),
                    max: Some(256)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Skill Level"),
                r#type: OptionType::Spin {
                    default: Some(20),
                    min: Some(0),
                    max: Some(20)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("Move Overhead"),
                r#type: OptionType::Spin {
                    default: Some(10),
                    min: Some(0),
                    max: Some(5000)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("nodestime"),
                r#type: OptionType::Spin {
                    default: Some(0),
                    min: Some(0),
                    max: Some(10000)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("UCI_Chess960"),
                r#type: OptionType::Check {
                    default: Some(false)
                },
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("UCI_LimitStrength"),
                r#type: OptionType::Check {
                    default: Some(false)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("UCI_Elo"),
                r#type: OptionType::Spin {
                    default: Some(1320),
                    min: Some(1320),
                    max: Some(3190)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("UCI_ShowWDL"),
                r#type: OptionType::Check {
                    default: Some(false)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("SyzygyPath"),
                r#type: OptionType::String {
                    default: Some(Cow::Borrowed("<empty>"))
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: Cow::Borrowed("SyzygyProbeDepth"),
                r#type: OptionType::Spin {
                    default: Some(1),
                    min: Some(1),
                    max: Some(100)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: "Syzygy50MoveRule".into(),
                r#type: OptionType::Check {
                    default: Some(true)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: "SyzygyProbeLimit".into(),
                r#type: OptionType::Spin {
                    default: Some(7),
                    min: Some(0),
                    max: Some(7)
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: "EvalFile".into(),
                r#type: OptionType::String {
                    default: Some("nn-1111cefa1111.nnue".into())
                }
            })
        );

        assert_eq!(
            options.next(),
            Some(Option {
                name: "EvalFileSmall".into(),
                r#type: OptionType::String {
                    default: Some("nn-37f18f62d772.nnue".into())
                }
            })
        );
        wait().await;
    }
}
