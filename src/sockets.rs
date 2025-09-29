use std::{collections::HashMap, sync::Arc};

use clap::Parser;
use futures::{SinkExt, StreamExt};
use parking_lot::Mutex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes, client::IntoClientRequest},
};

use crate::{opts::CliOpts, utils::FEED_WS_URL};

// TODO: alloc 5k for each coin
lazy_static::lazy_static! {
    pub static ref ws_messages: Arc<Mutex<HashMap<String, AllocRingBuffer<WsMessage>>>> =
                Arc::new(Mutex::new(HashMap::new()));
}

crate::pub_fields! {
    #[derive(Debug, Clone, Deserialize, Serialize, Default)]
    struct WsMessage {
        /// The type of the message
        r#type: String,
        /// Gets increased by every message
        sequence: usize,
        /// The product that this message comes from
        product_id: String,
        /// The current Price
        price: String,

        open_24h: String,
        /// The total trading volume in the past 24 hours
        volume_24h: String,
        /// The lowest price in the last 24 hours
        low_24h: String,
        /// The highest price in the last 24 hours
        high_24h: String,

        volume_30d: String,
        /// The best bid to the current price
        best_bid: String,
        /// the volume of the best bid
        best_bid_size: String,
        /// The best ask price
        best_ask: String,
        /// The volume of the best ask price
        best_ask_size: String,
        /// if if sold or buyed
        side: String,
        /// The time as an ISO 8601 timestring eg. 2022-10-19T23:28:22.061769Z
        time: String,
        /// The corresponding id to this transaction
        trade_id: usize,

        last_size: String,
    }
}

pub struct BaseSocket {}

impl BaseSocket {
    pub async fn connect(products: Vec<String>) -> anyhow::Result<()> {
        let req = FEED_WS_URL.into_client_request().unwrap();

        let (stream, _res) = connect_async(req).await.unwrap();
        let (mut tx, mut rx) = stream.split();

        let msg = Message::text(
            json!({
                  "type": "subscribe",
                  "channels": ["ticker"],
                  "product_ids": products
            })
            .to_string(),
        );

        tx.send(msg).await.unwrap();

        while let Some(msg) = rx.next().await {
            match msg?.clone() {
                Message::Text(m) => {
                    let _ = Self::handle_message(m).await;
                }
                Message::Ping(m) => tx.send(Message::Pong(m)).await?,
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_message(m: Utf8Bytes) -> anyhow::Result<()> {
        let msg = m.as_str();
        let p_msg: WsMessage = serde_json::from_str(msg)?;

        let mut l = ws_messages.lock();

        if !l.contains_key(&p_msg.product_id) {
            l.insert(
                p_msg.product_id,
                AllocRingBuffer::new(CliOpts::parse().watching.len() * 10000),
            );
            return Ok(());
        }

        l.get_mut(&p_msg.product_id).unwrap().enqueue(p_msg);

        Ok(())
    }
}
