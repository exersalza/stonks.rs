use lazy_static::lazy_static;
use ratatui::style::Color;
use std::collections::HashMap;

use crate::gradient_widget::GradientConfig;

pub const FEED_WS_URL: &'static str = "wss://ws-feed.exchange.coinbase.com";
pub const CURRENCIES: [&'static str; 2] = ["$", "€"];

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
