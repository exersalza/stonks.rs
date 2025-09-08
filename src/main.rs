use clap::Parser;
use ratatui::palette::cast::ComponentsInto;

use crate::{app::App, opts::CliOpts, sockets::BaseSocket};

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
    tokio::spawn(BaseSocket::connect());

    color_eyre::install()?;

    let opts = CliOpts::parse();

    let term = ratatui::init();
    let res = App::new(Some(opts.watching)).run(term).await;

    ratatui::restore();
    res
}
