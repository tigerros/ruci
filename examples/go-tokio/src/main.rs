//! This example immediately sends the quit message.
//! It's here to verify that the tokio integration works as expected.
//!
//! Requires that you have installed Stockfish.

use tokio::io::BufReader;
use tokio::process::{ChildStdin, ChildStdout};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut engine = ruci::Engine::<BufReader<ChildStdout>, ChildStdin>::from_path("stockfish")?;
    
    println!("== Sending quit message");
    engine.send_async(ruci::Quit).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
