use ruci::Engine;
use ruci::{engine, gui};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = Arc::new(Mutex::new(Engine::from_path("stockfish")?));
    let mut lock = engine.lock().await;

    println!("== Sending cui, waiting for uciok");

    let (_, _) = lock.use_uci().await?;

    println!("== Received uciok, sending isready");

    lock.is_ready().await?;

    println!("== Received readyok, starting analysis");

    drop(lock);

    let engine2 = engine.clone();
    let infos = Arc::new(Mutex::new(Vec::new()));
    let infos2 = infos.clone();

    let handle = tokio::spawn(async move {
        let mut engine_lock = engine2.lock().await;
        let mut infos_lock = infos2.lock().await;

        let info_fn = move |info: engine::Info| {
            // Ignore the insignificant ones
            if info.score.is_some() {
                let mut info = info.to_string();
                info.truncate(20);
                println!("Info #{}: {info}", infos_lock.len());
                infos_lock.push(info);
            }
        };

        engine_lock
            .go(
                gui::Go {
                    infinite: true,
                    ..Default::default()
                },
                info_fn,
            )
            .await
    });

    println!("== Waiting 5 secs");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("== Aborting task");
    handle.abort();
    println!("== Aborted");
    println!("== Task result: {:#?}", handle.await);

    let mut engine = Arc::into_inner(engine).unwrap().into_inner();
    let infos = Arc::into_inner(infos).unwrap().into_inner();

    println!("== Seen {} infos", infos.len());
    println!("== Sending quit message");

    engine.send(&gui::Quit.into()).await?;

    println!("== Sent. Program terminated");

    Ok(())
}
