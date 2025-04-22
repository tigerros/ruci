//! This example demonstrates a possible implementation of an engine.
//! All communication is done with UCI, using the [`Info`] message when another message is not
//! more appropriate.
//!
//! Accepts the following messages:
//! - [`Uci`](ruci::Uci)
//! - [`Position`](ruci::Position)
//! - [`Go`](ruci::Go) - no analysis, just outputs the first legal move [`shakmaty`] finds.
//!   Parameters are ignored except [`infinite`](ruci::Go#structfield.infinite).
//! - [`Quit`](ruci::Quit)

use ruci::gui::Message;
use ruci::{BestMove, Depth, Id, Info, NormalBestMove, UciOk, gui};
use shakmaty::Chess;
use shakmaty::uci::{IllegalUciMoveError, UciMove};
use shakmaty::{CastlingMode, Position};
use std::borrow::Cow;
use std::io;
use std::io::{BufRead, StdoutLock, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn info_string(stdout: &mut StdoutLock, s: Cow<str>) -> io::Result<()> {
    let info = Info {
        string: Some(s),
        ..Default::default()
    };

    writeln!(stdout, "{info}")
}

fn not_supported(stdout: &mut StdoutLock, message: &str) -> io::Result<()> {
    info_string(stdout, format!("{message} not supported").into())
}

struct State {
    position: Chess,
}

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut line = String::with_capacity(128);
    let mut state = State {
        position: Chess::new(),
    };

    info_string(&mut stdout, "engine started".into())?;

    loop {
        stdin.read_line(&mut line)?;
        let message = gui::Message::from_str(&line);
        line.clear();

        let message = match message {
            Ok(m) => m,
            Err(e) => {
                info_string(&mut stdout, format!("error parsing message: {e}").into())?;
                continue;
            }
        };

        match message {
            Message::Quit(_) => return Ok(()),
            Message::Position(position) => {
                let (position, moves) = match position {
                    ruci::Position::StartPos { moves } => (Chess::new(), moves),
                    ruci::Position::Fen { moves, fen } => {
                        match fen.into_owned().into_position(CastlingMode::Standard) {
                            Ok(p) => (p, moves),
                            Err(e) => {
                                info_string(&mut stdout, format!("error parsing FEN: {e}").into())?;
                                continue;
                            }
                        }
                    }
                };

                match moves.iter().try_fold(position, |mut position, r#move| {
                    let r#move = r#move.to_move(&state.position)?;
                    position.play_unchecked(&r#move);
                    Ok::<Chess, IllegalUciMoveError>(position)
                }) {
                    Ok(position) => {
                        state.position = position;
                        info_string(&mut stdout, "position set".into())?;
                    }
                    Err(e) => {
                        info_string(
                            &mut stdout,
                            format!("error converting UCI move to valid move: {e}").into(),
                        )?;
                    }
                }
            }
            Message::Go(go) => {
                let mut depth = 1;

                if let Some(r#move) = state.position.legal_moves().first() {
                    let info = Info {
                        depth: Some(Depth {
                            depth,
                            seldepth: None,
                        }),
                        pv: Cow::Owned(vec![r#move.to_uci(CastlingMode::Standard)]),
                        ..Default::default()
                    };
                    let best_move = BestMove::Normal(NormalBestMove {
                        r#move: r#move.to_uci(CastlingMode::Standard),
                        ponder: None,
                    });

                    writeln!(stdout, "{info}")?;
                    writeln!(stdout, "{best_move}")?;
                } else {
                    let null = BestMove::Normal(NormalBestMove {
                        r#move: UciMove::Null,
                        ponder: None,
                    });
                    writeln!(stdout, "{null}")?;
                }

                depth += 1;

                if go.infinite {
                    loop {
                        let info = Info {
                            depth: Some(Depth {
                                depth,
                                seldepth: None,
                            }),
                            string: Some("pretend that I'm doing something...".into()),
                            ..Default::default()
                        };
                        writeln!(stdout, "{info}")?;
                        depth += 1;
                        sleep(Duration::from_secs(5));
                    }
                }
            }
            Message::Uci(_) => {
                let id = Id::NameAndAuthor {
                    name: Cow::Borrowed("simpleton"),
                    author: Cow::Borrowed("tigerros"),
                };

                writeln!(stdout, "{id}")?;
                writeln!(stdout, "{UciOk}")?;
            }
            Message::Debug(_) => not_supported(&mut stdout, "debug")?,
            Message::SetOption(_) => not_supported(&mut stdout, "setoption")?,
            Message::Register(_) => not_supported(&mut stdout, "register")?,
            Message::IsReady(_) => not_supported(&mut stdout, "isread")?,
            Message::UciNewGame(_) => not_supported(&mut stdout, "ucinewgame")?,
            Message::Stop(_) => not_supported(&mut stdout, "stop")?,
            Message::PonderHit(_) => not_supported(&mut stdout, "ponderhit")?,
        }
    }
}
