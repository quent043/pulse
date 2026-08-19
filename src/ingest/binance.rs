use anyhow::Context;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::ingest::TradeSender;
use crate::model::{Exchange, Side, Trade};

#[derive(Deserialize)]
struct BinanceTrade {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "p")]
    price: String,
    #[serde(rename = "q")]
    quantity: String,
    #[serde(rename = "T")]
    trade_time_ms: u64,
    #[serde(rename = "m")]
    buyer_is_maker: bool,
}

impl TryFrom<BinanceTrade> for Trade {
    type Error = anyhow::Error;

    fn try_from(w: BinanceTrade) -> Result<Self, Self::Error> {
        Ok(Trade {
            exchange: Exchange::Binance,
            symbol: w.symbol,
            price: w.price.parse().context("price")?,
            quantity: w.quantity.parse().context("quantity")?,
            side: if w.buyer_is_maker { Side::Sell } else { Side::Buy },
            timestamp_ms: w.trade_time_ms,
        })
    }
}

pub async fn run(symbol: &str, tx: TradeSender) -> anyhow::Result<()> {
    let url = format!(
        "wss://stream.binance.com:9443/ws/{}@trade",
        symbol.to_lowercase()
    );

    let (mut stream, _) = connect_async(&url).await.context("connect")?;

    while let Some(msg) = stream.next().await {
        let msg = msg.context("stream")?;

        if let Message::Text(txt) = msg {
            let wire: BinanceTrade = serde_json::from_str(&txt).context("parse")?;
            let trade = Trade::try_from(wire)?;

            tx.send(trade).await.context("send")?;
        }
    }

    Ok(())
}
