// TODO: fix(?) having to send two newlines from the client after sending `Quit`.
//! This example demonstrates a TCP stream based engine, instead of the more standard
//! [`stdin`](io::stdin) and [`stdout`](io::stdout).
//!
//! Thanks to [`ruci`]'s generics, we can use TCP streams directly instead of having to spawn
//! a process and then route that through.
//!
//! To start the server, use `cargo run -p engine-server`.
//!
//! To start a client (and query the engine), you can use `cargo run -p engine-client`.
//! All that binary does is redirect `stdin` and `stdout`.
//! You can just as easily use [`ncat`](https://nmap.org/ncat) or something else (just check the [`ADDRESS`]).

use engine::engine;
use engine_server::ADDRESS;
use std::io::BufReader;
use std::net::TcpListener;
use std::{io, thread};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(ADDRESS)?;
    println!("listening on {ADDRESS}");

    loop {
        let (stream, address) = listener.accept()?;
        let write = stream.try_clone()?;
        let read = stream;
        println!("accepted client on {address}, delegating engine");

        thread::spawn(move || {
            let _ = engine(write, BufReader::new(read));
            println!("client from {address} disconnected");
        });
    }
}
