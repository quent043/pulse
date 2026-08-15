mod aggregate;
mod ingest;
mod model;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<model::Trade>(1024);

    let aggregator = tokio::spawn(aggregate::run(rx));

    let binance = tokio::spawn({
        let tx = tx.clone();
        async move { ingest::binance::run("BTCUSDT", tx).await }
    });

    drop(tx);

    binance.await??;
    aggregator.await?;
    Ok(())
}
