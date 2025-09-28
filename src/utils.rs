pub const FEED_WS_URL: &'static str = "wss://ws-feed.exchange.coinbase.com";

pub fn rotate_string(i:&mut String) -> String {
    // might aswell unwrap bc we know there has to be something inside the string
    format!("{}{}", i.pop().unwrap(), i)
}

