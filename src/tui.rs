// This file is more or less depricated



// Utils

// Structs

use terminal_size::terminal_size;
use std::io::{self, Write};

const ESC: &'static str = "\x1b";


pub const TOP_LEFT: char = '╭';
pub const TOP_RIGHT: char = '╮';
pub const BOTTOM_LEFT: char = '╰';
pub const BOTTOM_RIGHT: char = '╯';

pub const BORDER_VERT: char = '│';
pub const BORDER_HORI: char = '─';



pub struct Engine {
    screen: (u16, u16),
}

// Impls
impl Engine {
    pub fn new() -> Self {
        let (x, y) = terminal_size().unwrap_or((terminal_size::Width(0), terminal_size::Height(0)));

        Self { screen: (x.0, y.0) }
    }

    // getter
    pub fn get_screen_size(&self) -> (u16, u16) {
        self.screen
    }

    // functions
    // pubs
    pub fn render(&self) {
        Self::clear_screen()
    }

    pub fn rerender() {}

    pub fn place_cursor(&self, line: u16, column: u16) {
        print!("{ESC}[{line};{column}H")
    }

    pub fn clear(&self) {
        Self::clear_screen();
    }

    // priv
    fn create_borders() {}

    fn clear_screen() {
        print!("{ESC}[2J{ESC}[H");
        io::stdout().flush().unwrap()
    }



}
