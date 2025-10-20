use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use toml::value;

const CB_URL: &'static str = "https://api.exchange.coinbase.com/products";
const KK_URL: &'static str = "https://api.kraken.com/0/public/AssetPairs";

lazy_static::lazy_static! {
    pub static ref rew_cl: Arc<Mutex<reqwest::Client>> = Arc::new(Mutex::new(reqwest::Client::new()));
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WsOrigin {
    Coinbase,
    Kraken,
}

crate::pub_fields! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Pair {
        /// What coin are we talking about? Btc, Sol, Eth?
        name: String,
        /// The currency we trade in Eur, Usd, Usdc
        rl_currency: String,
        delimeter: String,
        /// These two might be same most of the time
        display_name: String,
        ws_name: String,
        ws_origin: WsOrigin,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct CoinbasePair {
        id: String,
        base_currency: String,
        quote_currency: String,
        status: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct KrakenPair {
        altname: String,
        wsname: String,
        base: String,
        quote: String,
        status: String,
    }
}

fn to_string(v: serde_json::value::Value) -> String {
    v.as_str().unwrap_or_default().to_string()
}

pub trait PairTrait {
    fn get_status(&self) -> String;
}

impl PairTrait for CoinbasePair {
    fn get_status(&self) -> String {
        self.status.to_lowercase()
    }
}

impl PairTrait for KrakenPair {
    fn get_status(&self) -> String {
        self.status.to_lowercase()
    }
}

fn parse_data<T>(data: Vec<T>, map_fn: impl FnMut(T) -> Pair) -> Vec<Pair>
where
    T: PairTrait,
{
    data.into_iter()
        // filter all out that are not online, bc we cant receive any data from them
        .filter(|f| &f.get_status() == "online")
        .map(map_fn)
        .collect()
}

pub async fn get_cb_pairs() -> anyhow::Result<Vec<Pair>> {
    let cl = rew_cl.lock();
    let res = match cl.get(CB_URL).send().await {
        Ok(v) => v,
        Err(e) => anyhow::bail!("[API] the request failed: {:?}", e),
    };

    let data: Vec<CoinbasePair> = match res.text().await {
        Ok(v) => {
            // this should not be able to panic as the api always returns valid json
            serde_json::from_str(&v).unwrap()
        }
        Err(e) => anyhow::bail!("[API] the returned text is invalid: {:?}", e),
    };

    let ret: Vec<Pair> = parse_data(data, |f| {
        let base = &f.base_currency;
        let quote = &f.quote_currency;

        Pair {
            name: base.clone(),
            rl_currency: quote.clone(),
            delimeter: f.id.replace(base, "").replace(quote, ""),
            display_name: f.id.clone(),
            ws_name: f.id.clone(),
            ws_origin: WsOrigin::Coinbase,
        }
    });

    Ok(ret)
}

pub async fn get_kk_pairs() -> anyhow::Result<Vec<Pair>> {
    let cl = rew_cl.lock();
    let res = match cl.get(KK_URL).send().await {
        Ok(v) => v,
        Err(e) => anyhow::bail!("[API] the request failed: {:?}", e),
    };

    let data: HashMap<String, serde_json::Value> = match res.text().await {
        Ok(v) => {
            // this should not be able to panic as the api always returns valid json
            serde_json::from_str(&v).unwrap()
        }
        Err(e) => anyhow::bail!("[API] the returned text is invalid: {:?}", e),
    };

    let mut ret = vec![];

    if let Some(values) = data["result"].as_object() {
        ret = values
            .values()
            .into_iter()
            .filter(|f| to_string(f["status"].clone()).to_lowercase() == "online")
            .map(|f| {
                let base = to_string(f["base"].clone());
                let quote = to_string(f["quote"].clone());

                Pair {
                    name: to_string(f["altname"].clone()),
                    rl_currency: quote.clone(),
                    delimeter: to_string(f["wsname"].clone())
                        .replace(&quote, "")
                        .replace(&base, ""),
                    display_name: to_string(f["altname"].clone()),
                    ws_name: to_string(f["wsname"].clone()),
                    ws_origin: WsOrigin::Kraken,
                }
            })
            .collect();
    }

    Ok(ret)
}
