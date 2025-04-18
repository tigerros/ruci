#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut engine = ruci::EngineAsync::from_path("stockfish")?;

    println!("== Sending quit message");
    engine.send(ruci::Quit).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
