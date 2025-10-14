use std::{collections::HashMap, fmt::Display, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use parking_lot::Mutex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tokio::time::interval;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes, client::IntoClientRequest},
};

use crate::{opts::CliOpts, utils::FEED_WS_URL};

// TODO: alloc 5k for each coin
lazy_static::lazy_static! {
    pub static ref connected: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref last_heartbeat: Arc<Mutex<DateTime<Utc>>> = Arc::new(Mutex::new(chrono::Utc::now()));
    pub static ref ws_messages: Arc<Mutex<HashMap<String, AllocRingBuffer<WsMessage>>>> =
                Arc::new(Mutex::new(HashMap::new()));
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Serialize, EnumIter, PartialEq, Eq, Debug, Clone, Default, Display)]
pub enum Type {
    #[default]
    ticker,
    heartbeat,
    subscriptions,
}

crate::pub_fields! {
    #[derive(Debug, Clone, Deserialize, Serialize, Default)]
    struct WsMessage {
        /// The type of the message
        r#type: Type,
        /// Gets increased by every message
        sequence: Option<usize>,
        /// The product that this message comes from
        product_id: Option<String>,
        /// The current Price
        price: Option<String>,

        open_24h: Option<String>,
        /// The total trading volume in the past 24 hours
        volume_24h: Option<String>,
        /// The lowest price in the last 24 hours
        low_24h: Option<String>,
        /// The highest price in the last 24 hours
        high_24h: Option<String>,

        volume_30d: Option<String>,
        /// The best bid to the current price
        best_bid: Option<String>,
        /// the volume of the best bid
        best_bid_size: Option<String>,
        /// The best ask price
        best_ask: Option<String>,
        /// The volume of the best ask price
        best_ask_size: Option<String>,
        /// if if sold or buyed
        side: Option<String>,
        /// The time as an ISO 8601 timestring eg. 2022-10-19T23:28:22.061769Z
        time: Option<String>,
        /// The corresponding id to this transaction
        trade_id: Option<usize>,

        last_size: Option<String>,
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
                  "channels": Type::iter().filter(|f| *f != Type::subscriptions).map(|f| format!("{f}")).collect::<Vec<String>>(),
                  "product_ids": products
            })
            .to_string(),
        );

        tx.send(msg).await.unwrap();


        tokio::spawn(Self::check_heartbeat());

        while let Some(msg) = rx.next().await {
            match msg?.clone() {
                Message::Text(m) => {
                    match Self::handle_message(m).await {
                        Ok(_) => (),
                        Err(e) => {
                            dbg!(e);
                        }
                    };
                }
                Message::Ping(m) => tx.send(Message::Pong(m)).await?,
                _ => {}
            }
        }

        Ok(())
    }


    async fn check_heartbeat() -> ! {
        let mut inter = interval(Duration::from_secs(1));

        loop {
            // check for heartbeat here
            let now = chrono::Utc::now();

            let delta = *last_heartbeat.lock() - now;

            dbg!(delta);

            inter.tick().await;
        }
    }

    async fn handle_message(m: Utf8Bytes) -> anyhow::Result<()> {
        let msg = m.as_str();
        let p_msg: WsMessage = serde_json::from_str(msg)?;

        let mut l = ws_messages.lock();

        match p_msg.r#type {
            Type::ticker => {
                let prod_id = p_msg.product_id.clone().unwrap();

                if !l.contains_key(&prod_id) {
                    l.insert(
                        prod_id,
                        AllocRingBuffer::new(CliOpts::parse().watching.len() * 10000),
                    );
                    return Ok(());
                }

                l.get_mut(&prod_id).unwrap().enqueue(p_msg);
            }
            Type::heartbeat => {
                let time = p_msg
                    .time
                    .clone()
                    .unwrap()
                    .parse::<DateTime<chrono::Utc>>()
                    .unwrap();
                *last_heartbeat.lock() = time;
            }

            Type::subscriptions => {
                *connected.lock() = true;
            }
        }

        Ok(())
    }
}
