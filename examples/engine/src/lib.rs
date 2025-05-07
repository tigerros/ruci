//! This example shows how to make a "portable" engine, which can easily be used in various
//! I/O situations.
//!
//! - See `engine-stdio` for an implementation using [`stdin`](io::stdin) and [`stdout`](io::stdout).
//! - See `engine-server` for a TCP stream implementation.
//!
//! # Specifications
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
use ruci::{BestMove, Depth, Gui, Id, Info, NormalBestMove, UciOk};
use shakmaty::uci::{IllegalUciMoveError, UciMove};
use shakmaty::{CastlingMode, Chess, Position};
use std::borrow::Cow;
use std::io;
use std::io::{BufRead, Write};
use std::thread::sleep;
use std::time::Duration;

struct State {
    position: Chess,
}

/// Starts a new engine that forever reads messages, unless told to quit.
pub fn engine<E, G>(engine: E, gui: G) -> io::Result<()>
where
    E: Write,
    G: BufRead,
{
    let mut gui = Gui { engine, gui };
    let mut state = State {
        position: Chess::new(),
    };

    gui.send_string("engine started")?;

    loop {
        let message = gui.read();

        let message = match message {
            Ok(m) => m,
            Err(e) => {
                gui.send_string(&e.to_string())?;
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
                                gui.send_string(&format!("error parsing FEN: {e}"))?;
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
                        gui.send_string("position set")?;
                    }
                    Err(e) => {
                        gui.send_string(&format!("error converting UCI move to valid move: {e}"))?;
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

                    gui.send(info)?;
                    gui.send(best_move)?;
                } else {
                    let null = BestMove::Normal(NormalBestMove {
                        r#move: UciMove::Null,
                        ponder: None,
                    });
                    gui.send(null)?;
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
                        gui.send(info)?;
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

                gui.send(id)?;
                gui.send(UciOk)?;
            }
            _ => gui.send_string("unsupported message")?,
        }
    }
}
