use clap::Parser;

use crate::{
    app::App,
    events::EventHandler,
    opts::CliOpts,
    sockets::{BaseSocket, WsMessage},
};

mod opts;
mod sockets;
mod tui;
mod utils;
mod crypto;

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
   // color_eyre::install()?;

    let mut coins = opts.watching.clone();

    let mut cb = tokio::spawn(BaseSocket::connect_cb(coins.clone()));
    let mut kk = tokio::spawn(BaseSocket::connect_kk());

    loop {
        tokio::select! {
            res = &mut cb => {
                cb = tokio::spawn(BaseSocket::connect_cb(coins.clone()));
            },
            res = &mut kk => {
                kk = tokio::spawn(BaseSocket::connect_kk());
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
        }
    }

    /*
    let opts = CliOpts::parse();

    let term = ratatui::init();

    let app = App::new(Some(opts.watching));

    let res = app.run(term).await;

    ratatui::restore(); */
    // res
    //
    // Ok(())
}
