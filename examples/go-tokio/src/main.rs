use ruci::gui::Quit;
use ruci::EngineAsync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut engine = EngineAsync::from_path("stockfish")?;

    println!("== Sending quit message");
    engine.send(Quit).await?;
    println!("== Sent. Program terminated");

    Ok(())
}
