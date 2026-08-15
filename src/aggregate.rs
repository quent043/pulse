use std::collections::HashMap;

use tokio::sync::mpsc::Receiver;

use crate::model::{Exchange, Trade};

#[derive(Default)]
pub struct State {
    last_price: HashMap<(Exchange, String), f64>,
}

pub async fn run(mut rx: Receiver<Trade>) {
    let mut state = State::default();

    while let Some(trade) = rx.recv().await {
        state
            .last_price
            .insert((trade.exchange, trade.symbol.clone()), trade.price);

        println!("{trade:?}");
    }
}
