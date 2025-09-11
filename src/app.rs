use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    events::{AppEvent, Event, EventHandler},
    sockets::fff,
};

use chrono::{DateTime, Local};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers}, layout::{Constraint, Layout, Rect}, style::{Color, Style, Stylize}, symbols, text::Text, widgets::{Axis, Block, Borders, Chart, Dataset}, DefaultTerminal, Frame
};
use ringbuffer::RingBuffer;

pub const UPPER_COLOR_BOUND: u8 = 255;
pub const LOWER_COLOR_BOUND: u8 = 150;
/// This should be easily subtractable/ addable to the two values above be. Otherwise the type goes
/// out of bounds and the program panics. So if the bounds are 150-255 you should use either 1 or 5
/// etc.
pub const CHANGE_COLOR_BY: u8 = 5;

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
    /// Whenever the app got started
    pub start_time: u64,

    /// Should the borders of the charts be animated or not
    pub border_animation: bool,

    /// this is a value that gets changed every tick by 1 plus or minus.
    /// this does have bounds tho, from 240-255
    color: u8,
    /// this decides if we add or subtract
    color_add: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            watching: vec!["SOL-USDC".to_string()],
            running: true,
            events: EventHandler::new(),
            start_time: Self::now(),
            border_animation: true,
            color: 255,
            color_add: false,
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

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards???")
            .as_secs()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                if self.watching.len() == 0 {
                    frame.render_widget(
                        Text::from("You dont have any Coins selected").centered(),
                        frame.area(),
                    );
                    return;
                }
                // TODO: add layouts for differente screen sizes and for the amount of chains to
                // watch
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
                    AppEvent::WSMessage(m) => {}
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn render_chart(&self, frame: &mut Frame, area: Rect, coin: String) {
        /* frame.render_widget(
            Block::bordered()
                .style(Color::Rgb(
                    if self.border_animation {
                        self.color
                    } else {
                        255
                    },
                    0,
                    100,
                ))
                .border_type(BorderType::Rounded)
                .title(coin.gray().into_centered_line()),
            area,
        ); */

        // add filtering for coins

        let data = fff.lock().iter().map(|i| {
            let time = i.time.parse::<DateTime<chrono::Utc>>().unwrap();
            (i.price, time.with_timezone(&Local).timestamp_millis() as f64)
        }).collect::<Vec<(f64, f64)>>();

        let x_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, 500.0])
            .labels(["0.0", "5.0", "10.0"]);

        // Create the Y axis and define its properties
        let y_axis = Axis::default()
            .style(Style::default().white())
            .bounds([0.0, 250000.0])
            .labels(["0.0", "5.0", "10.0"]);

        /* let data = (1..500)
            .map(|i: u64| (i as f64, i.pow(2) as f64))
            .collect::<Vec<(f64, f64)>>(); */

        let chart = Chart::new(vec![
            Dataset::default()
                .style(Color::Rgb(self.color, 0, 100))
                .marker(symbols::Marker::Braille)
                .data(&data),
        ])
        .block(
            Block::new()
                .title(format!("cahrt - {}", fff.lock().len()))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .borders(Borders::all())
                .style(Style::new().red()),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

        frame.render_widget(chart, area);
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
    pub fn tick(&mut self) {
        let lock = fff.lock();

        self.calc_color();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Calculate next step
    ///
    /// this also breaks if we subtract/add too much to the color and it goes out of type bounds
    /// TODO: add out of type bounds checker
    fn calc_color(&mut self) {
        // check bounds
        if self.color <= LOWER_COLOR_BOUND {
            self.color_add = true;
        }

        if self.color >= UPPER_COLOR_BOUND {
            self.color_add = false;
        }

        if self.color_add {
            self.color += CHANGE_COLOR_BY;
            return;
        }

        self.color -= CHANGE_COLOR_BY;
    }
}
