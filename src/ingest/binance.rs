use crate::ingest::TradeSender;

pub async fn run(symbol: &str, tx: TradeSender) -> anyhow::Result<()> {
    let _ = (symbol, tx);
    todo!("Binance WS ingestion")
}
