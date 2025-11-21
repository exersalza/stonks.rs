use clap::Parser;

use crate::{
    app::App,
    crypto::{get_cb_pairs, get_kk_pairs},
    events::EventHandler,
    opts::CliOpts,
    sockets::{ws_connected, BaseSocket, WsMessage}, utils::CoinCache,
};

mod crypto;
mod opts;
mod sockets;
mod tui;
mod utils;

pub mod app;
pub mod events;
pub mod macros;
pub mod memes;
pub mod ui;

/// Widgets
pub mod gradient_widget;


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let opts = CliOpts::parse();
    color_eyre::install()?;

    let mut coins = opts.watching.clone();

    if coins.len() > 0 {

    }


    println!("[DEBUG] fetching coinbase pairs...");
    // let coinbase_pairs = get_cb_pairs().await.unwrap();
    let cc = CoinCache::new();
    // let _ = dbg!(cc.sync_to_file());


    return Ok(());

    println!("[DEBUG] fetching kraken pairs...");
    // let kraken_pairs = get_kk_pairs().await.unwrap();

    println!("[DEBUG] starting coinbase websocket...");
    let mut cb = tokio::spawn(BaseSocket::connect_cb(coins.clone()));
    println!("[DEBUG] starting kraken websocket...");
    let mut kk = tokio::spawn(BaseSocket::connect_kk());

    {
        let mut f = ws_connected.lock();
        f.kraken = true;
        f.coinbase = true;
    }

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut cb => {
                    let mut l = ws_connected.lock();
                    l.coinbase = false;

                    cb = tokio::spawn(BaseSocket::connect_cb(coins.clone()));
                },
                _ = &mut kk => {
                    let mut l = ws_connected.lock();
                    l.kraken = false;

                    kk = tokio::spawn(BaseSocket::connect_kk());
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
            }
        }
    });

    let opts = CliOpts::parse();
    let term = ratatui::init();
    let app = App::new(Some(opts.watching));
    let res = app.run(term).await;

    ratatui::restore();
    res
    // Ok(())
}
