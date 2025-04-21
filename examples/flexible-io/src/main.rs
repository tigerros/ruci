//! This example shows the flexibility of the [`Engine`] struct, by: 
//! - Using other [`BufRead`](std::io::BufRead) and [`Write`](std::io::Write) types rather than
//!   [`ChildStdout`](std::process::ChildStdout) and [`ChildStdin`](std::process::ChildStdin)
//!   like in the other examples.
//! - Interchangeably using sync/async functions. Note that this works only for types where the [`std`]
//!   and [`tokio`] traits overlap.

use ruci::{Engine, Id};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("1. Setting input to id and uciok");
    let r#in = &mut b"id name stringerton author tigerros\nuciok\n".as_slice();
    let out = Vec::<u8>::new();

    let mut engine = Engine {
        r#in,
        out,
        strict: false
    };

    println!("1. Sending uci");
    let id = engine.use_uci(|_| {})?;
    println!("1. Received uciok and id: {id:#?}");
    assert_eq!(id, Some(Id::NameAndAuthor {
        name: "stringerton".into(),
        author: "tigerros".into()
    }));
    println!("1. Output bytes: {:?}", engine.out);
    
    println!();

    println!("2. Setting input to readyok");
    *engine.r#in = b"readyok\n";
    println!("2. Sending isready");
    engine.is_ready_async().await?;
    println!("2. Received readyok");
    println!("2. Output bytes: {:?}", engine.out);

    Ok(())
}
