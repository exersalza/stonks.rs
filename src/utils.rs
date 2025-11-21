use lazy_static::lazy_static;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::fs::remove_file;
use std::{
    collections::HashMap,
    env::temp_dir,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    crypto::{CoinbasePair, KrakenPair, Pair},
    gradient_widget::GradientConfig,
};

pub const CB_FEED_URL: &'static str = "wss://ws-feed.exchange.coinbase.com";
pub const KK_WS_URL: &'static str = "wss://ws.kraken.com/v2";
pub const CURRENCIES: [&'static str; 5] = ["$", "€", "£", "¥", "Fr"];

lazy_static! {
    pub static ref CRYPTO_COLOR_CODES: HashMap<String, GradientConfig> = HashMap::from([
        (
            "SOL".to_string(),
            GradientConfig::new_4(
                Color::Rgb(154, 69, 254),
                Color::Rgb(87, 152, 203),
                Color::Rgb(21, 240, 150),
                Color::Rgb(87, 152, 203),
            )
        ),
        (
            "BTC".to_string(),
            GradientConfig::new_1(Color::Rgb(247, 147, 26)) // Bitcoin Orange
        ),
        (
            "ETH".to_string(),
            GradientConfig::new_1(Color::Rgb(72, 203, 217)) // Ethereum Blue
        ),
        (
            "ENA".to_string(),
            GradientConfig::new_2(Color::Rgb(30, 30, 30), Color::Rgb(200, 200, 200))
        ),
        (
            "ADA".to_string(),
            GradientConfig::new_2(Color::Rgb(0, 84, 81), Color::Rgb(28, 191, 191)) // Cardano Teal Gradient
        ),
        (
            "XRP".to_string(),
            GradientConfig::new_1(Color::Rgb(0, 70, 143)) // Ripple Blue
        ),
        (
            "LTC".to_string(),
            GradientConfig::new_1(Color::Rgb(191, 191, 191)) // Litecoin Silver
        ),
        (
            "DOT".to_string(),
            GradientConfig::new_2(Color::Rgb(29, 29, 27), Color::Rgb(222, 0, 53)) // Polkadot Black to Red
        ),
        (
            "DOGE".to_string(),
            GradientConfig::new_2(Color::Rgb(194, 153, 57), Color::Rgb(255, 197, 64)) // Dogecoin Gold Gradient
        ),
        (
            "AVAX".to_string(),
            GradientConfig::new_1(Color::Rgb(255, 0, 0)) // Avalanche Red
        ),
        (
            "MATIC".to_string(),
            GradientConfig::new_2(Color::Rgb(149, 45, 183), Color::Rgb(86, 48, 140)) // Polygon Purple Gradient
        ),
        (
            "BCH".to_string(),
            GradientConfig::new_1(Color::Rgb(190, 224, 103)) // Bitcoin Cash Green
        ),
        (
            "LINK".to_string(),
            GradientConfig::new_1(Color::Rgb(16, 126, 229)) // Chainlink Blue
        ),
        (
            "UNI".to_string(),
            GradientConfig::new_1(Color::Rgb(102, 51, 153)) // Uniswap Purple
        ),
        (
            "FTT".to_string(),
            GradientConfig::new_1(Color::Rgb(144, 27, 169)) // FTX Purple
        ),
        (
            "FIL".to_string(),
            GradientConfig::new_2(Color::Rgb(255, 85, 85), Color::Rgb(255, 153, 153)) // Filecoin Red Shades
        ),
        (
            "VET".to_string(),
            GradientConfig::new_2(Color::Rgb(0, 153, 204), Color::Rgb(0, 51, 102)) // VeChain Blues
        ),
        (
            "EOS".to_string(),
            GradientConfig::new_2(Color::Rgb(57, 57, 57), Color::Rgb(1, 1, 1)) // EOS Grey to Black
        ),
        (
            "MKR".to_string(),
            GradientConfig::new_1(Color::Rgb(26, 171, 155)) // Maker Teal
        ),
        (
            "SKY".to_string(),
            GradientConfig::new_2(
                Color::Rgb(48, 90, 224),   // Skycoin Blue
                Color::Rgb(102, 153, 255)  // Lighter Blue for gradient
            )
        )
    ]);
}

pub fn rotate_string(i: &mut String) -> String {
    // might aswell unwrap bc we know there has to be something inside the string
    format!("{}{}", i.pop().unwrap(), i)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinCache {
    pub cb_thread: bool,
    pub kk_thread: bool,
    file_location: PathBuf,
    pairs: Vec<Pair>,
    check_str: String,
}

#[allow(unused)]
impl CoinCache {
    pub fn new() -> Self {
        let pairs = vec![];

        let mut f = Self {
            cb_thread: false,
            kk_thread: false,
            file_location: temp_dir(),
            pairs,
            check_str: base64::encode("stonks.rs_stonks.rs"),
        };

        // this is fkcing bs what i'm doing, but fck it
        if f.gen_file_name().is_file() {
            match f.sync_from_file() {
                Err(e) => println!("[DEBUG] file seems to be broken idk {e}"),
                _ => return f,
            }
        }

        let mut temp_file_cache = vec![];
        for i in std::fs::read_dir(temp_dir()).unwrap() {
            let entry = i.unwrap();

            let entry_name = entry.path().display().to_string();
            if entry_name.contains(&f.check_str) {
                temp_file_cache.push(entry_name);
            }
        }


        // remove old files, can also be other files with containing the same base64 string
        temp_file_cache.iter().for_each(|f| {
            // we dont care about errors in this case as we just ballin
            let _ = std::fs::remove_file(f); // !important !important !important
        });

        f
    }

    pub fn sync_to_file(&self) -> anyhow::Result<()> {
        let filename = self.gen_file_name();

        if filename.is_file() {
            anyhow::bail!("File already exists")
        }

        let mut f = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(filename)?;

        let data = json!(self.pairs).to_string();

        match f.write(data.as_bytes()) {
            Err(e) => anyhow::bail!(e),
            _ => (),
        };

        Ok(())
    }

    pub fn sync_from_file(&mut self) -> anyhow::Result<()> {
        let filename = self.gen_file_name();

        let mut f = OpenOptions::new().read(true).open(filename)?;

        let mut buf = String::new();

        f.read_to_string(&mut buf)?;
        self.pairs = serde_json::from_str(&buf)?;

        Ok(())
    }

    ///
    ///
    /// # Returns
    ///
    ///
    fn gen_file_name(&self) -> PathBuf {
        let mark = self.check_str.clone();
        let today = chrono::Utc::now().format("%Y-%d-%m").to_string();

        PathBuf::from(format!("{}{today}{mark}", temp_dir().display()))
    }
}
