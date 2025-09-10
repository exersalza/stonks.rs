use crate::{
    events::{AppEvent, Event, EventHandler},
    sockets::{WsMessage, fff},
};

use colorgrad;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, text::Text, widgets::{Block, BorderType}, DefaultTerminal, Frame
};
use ringbuffer::RingBuffer;
use tui_gradient_block::{
    gradient_block::{GradientBlock, Position},
    theme_presets::multi_color::t_colorgrad_warm,
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// What chains/coins should be watched
    /// NOTE: this is just used to show it on the frontend for user information
    pub watching: Vec<String>,
    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            watching: vec!["SOL-USDC".to_string()],
            running: true,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(watching: Option<Vec<String>>) -> Self {
        match watching {
            Some(v) => Self {
                watching: v,
                ..Default::default()
            },
            _ => Self::default(),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                if self.watching.len() == 0 {
                    frame.render_widget(Text::from("You dont have any Coins selected").centered(), frame.area());
                    return
                }

                let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());

                self.render_chart(frame, top, self.watching[0].clone());
            })?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::WSMessage(m) => {
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn render_chart(&self, frame: &mut Frame, area: Rect, coin: String) {
        frame.render_widget(
            Block::bordered()
                .style(Color::Rgb(255, 0, 100))
                .border_type(BorderType::Rounded)
                .title(coin.gray().into_centered_line()),
            area,
        );
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {
        let lock = fff.lock();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
