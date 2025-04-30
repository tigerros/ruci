// TODO: fix(?) having to send two newlines from the client after sending `Quit`.
//! This example demonstrates a possible implementation of a [`tokio`]-based engine (server).
//!
//! Instead of using `stdin` or `stdout` like in the `engine` example, we're using a [`TcpListener`]
//! server to start a new engine for every connection.
//! Thanks to [`ruci`]'s generics, we can use [`TcpStream`](tokio::net::TcpStream)s directly.
//!
//! So, you know, if you wanted to make a network based engine with minimal effort, you can.
//!
//! To start the server, use `cargo run -p engine-server`.
//!
//! To start a client (and query the engine), you can use `cargo run -p engine-client`.
//! All that binary does is redirect `stdin` and `stdout`.
//! You can just as easily use [`ncat`](https://nmap.org/ncat) or something else (just check the [`ADDRESS`]).
//!
//! Accepts the following messages:
//! - [`Uci`](ruci::Uci)
//! - [`Position`](ruci::Position)
//! - [`Go`](ruci::Go)
//! - [`Quit`](ruci::Quit)

use engine_server::ADDRESS;
use ruci::gui::Message;
use ruci::{BestMove, Gui, Id, NormalBestMove, UciOk};
use shakmaty::uci::UciMove;
use shakmaty::{Chess, Move, Position, Role, Square};
use std::borrow::Cow;
use tokio::io::BufReader;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

const ID: Id = Id::NameAndAuthor {
    name: Cow::Borrowed("serverton"),
    author: Cow::Borrowed("me"),
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(ADDRESS).await?;
    println!("listening on {ADDRESS}");

    loop {
        let (stream, address) = listener.accept().await?;
        let (read, write) = stream.into_split();
        println!("accepted client on {address}, delegating engine");

        tokio::spawn(async move {
            let _ = engine(write, read).await;
            println!("client from {address} disconnected");
        });
    }
}

async fn engine<E, G>(engine: E, gui: G) -> anyhow::Result<()>
where
    E: AsyncWrite + Unpin,
    G: AsyncRead + Unpin,
{
    let mut gui = Gui {
        engine,
        gui: BufReader::new(gui),
    };
    let mut bongcloud_position = Chess::new();

    bongcloud_position.play_unchecked(&Move::Normal {
        from: Square::E2,
        to: Square::E4,
        capture: None,
        promotion: None,
        role: Role::Pawn,
    });
    bongcloud_position.play_unchecked(&Move::Normal {
        from: Square::E7,
        to: Square::E5,
        capture: None,
        promotion: None,
        role: Role::Pawn,
    });

    let mut current_position = Chess::new();

    loop {
        let message = gui.read_async().await;

        let message = match message {
            Ok(m) => m,
            Err(e) => {
                gui.send_string_async(&e.to_string()).await?;
                continue;
            }
        };

        match message {
            Message::Quit(_) => return Ok(()),
            Message::Uci(_) => {
                gui.send_async(ID).await?;
                gui.send_async(UciOk).await?;
            }
            Message::Position(ruci::Position::StartPos { moves }) => {
                let mut position = Chess::new();

                for r#move in moves.iter() {
                    let r#move = match r#move.to_move(&position) {
                        Ok(r#move) => r#move,
                        Err(e) => {
                            gui.send_string_async(&e.to_string()).await?;
                            continue;
                        }
                    };

                    position.play_unchecked(&r#move);
                }

                current_position = position;
            }
            Message::Position(ruci::Position::Fen { .. }) => {
                gui.send_string_async("setting FEN is not supported")
                    .await?;
            }
            Message::Go(_) => {
                if current_position != bongcloud_position {
                    gui.send_string_async(
                        "unsupported position. Position must follow the moves 1. e4 e5",
                    )
                    .await?;
                } else {
                    // I'm so fucking funny
                    gui.send_async(BestMove::Normal(NormalBestMove {
                        r#move: UciMove::Normal {
                            from: Square::E1,
                            to: Square::E2,
                            promotion: None,
                        },
                        ponder: None,
                    }))
                    .await?
                }
            }
            _ => gui.send_string_async("unsupported message").await?,
        }
    }
}
