//! This is the client of the engine server. See the `server` example docs.

use engine_server::ADDRESS;
use std::io::{stdin, stdout};
use std::net::TcpStream;
use std::{io, thread};

fn main() -> anyhow::Result<()> {
    let mut engine = TcpStream::connect(ADDRESS)?;
    let mut gui = engine.try_clone()?;

    println!("connected to {ADDRESS}");

    thread::scope(|s| {
        s.spawn(move || io::copy(&mut stdin().lock(), &mut gui));
        s.spawn(move || io::copy(&mut engine, &mut stdout().lock()));
    });

    Ok(())
}
