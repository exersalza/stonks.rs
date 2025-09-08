use core::fmt;

use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{client::IntoClientRequest, Message, Utf8Bytes},
};

use crate::utils::FEED_WS_URL;


pub struct BaseSocket {}

impl BaseSocket {
    pub fn new<T: fmt::Display>(url: T) -> Self {
        Self {}
    }

    pub async fn connect() -> anyhow::Result<()> {
        let req = FEED_WS_URL.into_client_request().unwrap();

        let (stream, _res) = connect_async(req).await.unwrap();
        let (mut tx, mut rx) = stream.split();

        let msg = Message::text(
            json!({
                  "type": "subscribe",
                  "channels": ["ticker"],
                  "product_ids": ["SOL-USD"]
            })
            .to_string(),
        );

        tx.send(msg).await.unwrap();

        while let Some(msg) = rx.next().await {
            match dbg!(msg).unwrap().clone() {
                Message::Text(m) => {
                    Self::handle_message(m).await;
                }
                Message::Ping(m) => tx.send(Message::Pong(m)).await?,
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_message(m: Utf8Bytes) {}
}
