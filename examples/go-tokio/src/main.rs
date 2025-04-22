//! This example immediately sends the quit message.
//! It's here to verify that the tokio integration works as expected.
//!
//! Requires that you have installed Stockfish.

use ruci::Engine;
use std::process::Stdio;
use tokio::io::BufReader;
use tokio::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut process = Command::new("stockfish")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    let stdout = process.stdout.take().unwrap();
    let stdin = process.stdin.take().unwrap();

    let mut engine = Engine {
        r#in: BufReader::new(stdout),
        out: stdin,
        strict: false,
    };

    println!("== Sending quit message and waiting for engine process");
    engine.send_async(ruci::Quit).await?;
    process.wait().await?;
    println!("== Sent. Program terminated");

    Ok(())
}
