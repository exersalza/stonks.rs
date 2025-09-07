use clap::Parser;

use crate::{app::App, opts::CliOpts};

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
    color_eyre::install()?;

    let opts = CliOpts::parse();

    let term = ratatui::init();
    let res = App::new(Some(opts.watching)).run(term).await;


    ratatui::restore();
    res
}
