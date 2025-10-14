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

    let thread = tokio::spawn(BaseSocket::connect(opts.watching));
    let _ = thread.await.inspect(|i| {
        dbg!(i);
    });

    /*
    let opts = CliOpts::parse();

    let term = ratatui::init();

    let app = App::new(Some(opts.watching));

    let res = app.run(term).await;

    ratatui::restore(); */
    // res
    //
    Ok(())
}
