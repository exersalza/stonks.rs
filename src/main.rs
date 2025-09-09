use clap::Parser;

use crate::{app::App, events::EventHandler, opts::CliOpts, sockets::{BaseSocket, WsMessage}};

mod macros;
mod opts;
mod sockets;
mod tui;
mod utils;

pub mod app;
pub mod events;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let opts = CliOpts::parse();
    color_eyre::install()?;

    tokio::spawn(BaseSocket::connect(opts.watching));

    let opts = CliOpts::parse();

    let term = ratatui::init();

    let app = App::new( Some(opts.watching));


    let res = app.run(term).await;

    ratatui::restore();
    res
}
