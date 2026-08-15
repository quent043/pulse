pub mod binance;

use tokio::sync::mpsc::Sender;

use crate::model::Trade;

pub type TradeSender = Sender<Trade>;
