use std::{
    io::{stdin, stdout, Write},
    process::ExitCode, thread,
};

use crate::{sockets::BaseSocket, tui::{Engine, BORDER_HORI, BORDER_VERT, BOTTOM_LEFT, BOTTOM_RIGHT, TOP_LEFT, TOP_RIGHT}, utils::rotate_string};

mod macros;
mod sockets;
mod tui;
mod utils;


pub mod app;
pub mod events;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    
    let term = ratatui::init();
    let res = App::new().run(term).await;
    ratatui::restore();
    res
}
