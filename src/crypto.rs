use serde::{Deserialize, Serialize};

const CB_URL: &'static str = "https://api.exchange.coinbase.com/products";
const KK_URL: &'static str = "https://api.kraken.com/0/public/AssetPairs";

crate::pub_fields! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Coin {
        base_curreny: String,
        quote_currency: String,
        display_name: String,
    }
}

pub async fn get_cb_pairs() -> Vec<Coin> {
    let cl = reqwest::Client::new();
    let res = cl.get(CB_URL).send().await;

    todo!()
}

pub async fn get_kk_pais() -> Vec<Coin> {
    todo!()
}
