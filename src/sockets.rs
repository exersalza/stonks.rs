use std::{collections::VecDeque, sync::Arc};

use futures::{SinkExt, StreamExt};
use ringbuffer::{AllocRingBuffer, ConstGenericRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use parking_lot::Mutex;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes, client::IntoClientRequest},
};

use crate::utils::FEED_WS_URL;

lazy_static::lazy_static! {
    pub static ref fff: Arc<Mutex<AllocRingBuffer<WsMessage>>> = Arc::new(Mutex::new(AllocRingBuffer::new(5000)));
}

crate::pub_fields! {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct WsMessage {
        r#type: String,
        sequence: usize,
        product_id: String,
        price: String,
        open_24h: String,
        volume_24h: String,
        low_24h: String,
        high_24h: String,
        volume_30d: String,
        best_bid: String,
        best_bid_size: String,
        best_ask: String,
        best_ask_size: String,
        side: String,
        time: String,
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
        fff.lock().enqueue(p_msg);

        Ok(())
    }
}
