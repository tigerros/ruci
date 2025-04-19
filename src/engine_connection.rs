use crate::errors::{ConnectionError, ReadError, ReadWriteError};
use crate::Go;
use crate::{engine, gui};
use crate::{BestMove, Id, Info};
use core::fmt::Display;
use std::ffi::OsStr;
use std::io;
use std::io::{BufRead, BufReader, Write};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::process::{Child, ChildStdin, ChildStdout, Command};
use std::str::FromStr;

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

    // CLIPPY: Message is implemented for borrows as well
    #[allow(clippy::needless_pass_by_value)]
    /// Sends a message.
    ///
    /// # Errors
    /// See [`Write::write_all`].
    pub fn send<M>(&mut self, message: M) -> io::Result<()>
    where
        M: gui::traits::Message + Display,
    {
        self.stdin
            .write_all((message.to_string() + "\n").as_bytes())
    }

    /// Skips some lines.
    ///
    /// # Errors
    /// See [`BufRead::read_until`].
    pub fn skip_lines(&mut self, count: usize) -> io::Result<()> {
        let mut buf = Vec::with_capacity(512);

        for _ in 0..count {
            let bytes = self.stdout.read_until(b'\n', &mut buf)?;

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
    pub fn read<'m>(&mut self) -> Result<engine::Message<'m>, ReadError> {
        let mut line = String::new();

        if self.strict {
            loop {
                self.stdout.read_line(&mut line).map_err(ReadError::Io)?;

                if line.is_empty() || line.chars().all(char::is_whitespace) {
                    line.clear();
                    continue;
                }

                break;
            }

            engine::Message::from_str(&line).map_err(ReadError::Parse)
        } else {
            loop {
                self.stdout.read_line(&mut line).map_err(ReadError::Io)?;

                if let Ok(message) = engine::Message::from_str(&line) {
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
    pub fn use_uci<'m>(
        &mut self,
        mut option_receiver: impl FnMut(crate::Option<'m>),
    ) -> Result<Option<Id<'m>>, ReadWriteError> {
        self.send(crate::Uci).map_err(ReadWriteError::Write)?;

        let mut id = None::<Id>;

        loop {
            match self.read().map_err(ReadWriteError::Read)? {
                engine::Message::Option(option) => option_receiver(option),
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
    pub fn go<'m>(
        &mut self,
        message: &Go,
        mut info_receiver: impl FnMut(Info<'m>),
    ) -> Result<BestMove, ReadWriteError> {
        self.send(message).map_err(ReadWriteError::Write)?;

        loop {
            match self.read().map_err(ReadWriteError::Read)? {
                engine::Message::Info(info) => info_receiver(*info),
                engine::Message::BestMove(best_move) => {
                    return Ok(best_move);
                }
                _ => (),
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    /// Sends [`IsReady`](gui::IsReady) and waits for [`ReadyOk`](engine::ReadyOk).
    pub fn is_ready(&mut self) -> Result<(), ReadWriteError> {
        self.send(crate::IsReady).map_err(ReadWriteError::Write)?;

        loop {
            if let engine::Message::ReadyOk(_) = self.read().map_err(ReadWriteError::Read)? {
                return Ok(());
            }
        }
    }

    // Nice but probably too opinionated
    // /// Analyses a game using multiple threads and a closure that generates the engine to use.
    // ///
    // /// This function does the following:
    // /// 1. Build a list of positions from the given `game`.
    // /// 2. Create a thread for every `positions_per_thread` positions (7 if `None`).
    // /// 3. Using the engine from the given closure (`engine`) to analyze those positions.
    // /// 4. Report back every [`Info`] and [`BestMove`] along with the index of the analyze move,
    // ///    using the `receiver` closure.
    // /// 5. Return when all threads are joined.
    // ///
    // /// You can use some commands on the engine in the closure before you return it, e.g. send a message like [`Self::use_uci`],
    // /// if the engine you're using requires it.
    // ///
    // /// # Errors
    // /// Errors if the `engine` closure errored.
    // #[allow(clippy::missing_panics_doc)] // Doesn't actually panic
    // pub fn analyse_multithread<'go, Err>(
    //     game: impl Iterator<Item = UciMove>,
    //     engine: impl Fn() -> Result<Self, Err> + Sync,
    //     go: impl Fn() -> &'go Go<'go> + Sync,
    //     info_receiver: impl Fn(usize, Info) + Sync,
    //     best_move_receiver: impl Fn(usize, BestMove) + Sync,
    //     positions_per_thread: Option<NonZeroUsize>,
    // ) -> Result<(), Err>
    // where
    //     Err: Send + From<ReadWriteError>,
    // {
    //     let positions_per_thread = positions_per_thread.map_or(7, NonZeroUsize::get);
    //     let game_size = game.size_hint();
    //     let game_size = game_size.1.unwrap_or(game_size.0);
    //     let mut histories = Vec::with_capacity(game_size);
    //     let mut history = Vec::with_capacity(game_size);
    //
    //     for (i, r#move) in game.into_iter().enumerate() {
    //         history.push(r#move);
    //         histories.push((i, history.clone()));
    //     }
    //
    //     let engine = &engine;
    //     let go = &go;
    //     let info_receiver = &info_receiver;
    //     let best_move_receiver = &best_move_receiver;
    //
    //     thread::scope(|scope| {
    //         // CLIPPY: positions_per_thread is either NonZero or 7
    //         #[allow(clippy::arithmetic_side_effects)]
    //         let mut threads = Vec::with_capacity(game_size / positions_per_thread);
    //
    //         for history_chunk in histories.chunks(positions_per_thread) {
    //             threads.push(scope.spawn(move || -> Result<(), Err> {
    //                 let mut engine = engine()?;
    //
    //                 for (i, history) in history_chunk {
    //                     engine
    //                         .send(&Position::StartPos {
    //                             moves: Cow::Borrowed(history),
    //                         })
    //                         .map_err(ReadWriteError::Write)?;
    //                     let best_move = engine.go(go(), |info| info_receiver(*i, info))?;
    //                     best_move_receiver(*i, best_move);
    //                 }
    //
    //                 Ok(())
    //             }));
    //         }
    //
    //         for thread in threads {
    //             // CLIPPY: The thread can't panic
    //             #[allow(clippy::unwrap_used)]
    //             thread.join().unwrap()?;
    //         }
    //
    //         Ok(())
    //     })
    // }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::{NormalBestMove, OptionType};
    use alloc::borrow::Cow;
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
        Engine::from_path(ENGINE_EXE).unwrap()
    }
    
    #[test]
    fn is_ready() {
        let mut engine_conn = engine_conn();

        engine_conn.is_ready().unwrap();
    }
    
    // CLIPPY: It's literally used???
    #[test]
    #[allow(clippy::extra_unused_lifetimes)]
    fn lifetimes<'a>() {
        let mut engine_conn = engine_conn();

        if engine_conn.read().unwrap() == engine::Message::Option(crate::Option {
            name: Cow::Borrowed::<'a>(""),
            r#type: OptionType::Button
        }) {}
    }

    #[test]
    fn skip_lines() {
        let mut engine_conn = engine_conn();

        engine_conn.send(crate::Uci).unwrap();

        engine_conn.skip_lines(4).unwrap();

        let mut line = String::new();
        engine_conn.stdout.read_line(&mut line).unwrap();

        assert_eq!(
            line.trim(),
            "option name Debug Log File type string default <empty>"
        );
    }

    /// See the [`BestMove::Other`](BestMove::Other) docs for what this tests.
    #[test]
    fn analyze_checkmate() {
        let mut engine_conn = engine_conn();

        engine_conn.send(crate::Uci).unwrap();

        engine_conn
            .send(crate::Position::Fen {
                moves: Cow::Borrowed(&[]),
                fen: Cow::Owned(
                    Fen::from_ascii(
                        b"rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
                    )
                    .unwrap(),
                ),
            })
            .unwrap();

        let best_move = engine_conn
            .go(
                &Go {
                    depth: Some(5),
                    ..Default::default()
                },
                |_| {},
            )
            .unwrap();

        assert_eq!(best_move, BestMove::Other);
    }

    #[test]
    fn go() {
        let mut engine_conn = engine_conn();

        let best_move = engine_conn
            .go(
                &Go {
                    depth: Some(15),
                    ..Default::default()
                },
                |_| {},
            )
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

        engine_conn.send(crate::UciNewGame).unwrap();
        engine_conn
            .send(crate::Position::StartPos {
                moves: Cow::Borrowed(&[UciMove::from_ascii(b"d2d4").unwrap()]),
            })
            .unwrap();

        let best_move = engine_conn
            .go(
                &Go {
                    depth: Some(25),
                    ..Default::default()
                },
                |_| {},
            )
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
    #[test]
    fn use_uci() {
        use crate::{Id, Option};
        use core::fmt::Write;

        let mut engine_conn = engine_conn();

        let mut options = Vec::new();
        let id = engine_conn.use_uci(|option| options.push(option)).unwrap();

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
    }
}
