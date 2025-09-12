use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    events::{AppEvent, Event, EventHandler},
    gradient_widget::{GradientConfig, GradientWrapper},
    sockets::{WsMessage, fff},
};

use chrono::{DateTime, Local, TimeZone, Utc};
use lazy_static::lazy_static;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Styled, Stylize},
    symbols,
    text::{Text, ToLine},
    widgets::{Axis, Block, Borders, Chart, Dataset},
};
use ringbuffer::RingBuffer;

pub const UPPER_COLOR_BOUND: u8 = 255;
pub const LOWER_COLOR_BOUND: u8 = 150;
/// This should be easily subtractable/ addable to the two values above be. Otherwise the type goes
/// out of bounds and the program panics. So if the bounds are 150-255 you should use either 1 or 5
/// etc.
pub const CHANGE_COLOR_BY: u8 = 5;

lazy_static! {
    pub static ref CRYPTO_COLOR_CODES: HashMap<String, GradientConfig> = HashMap::from([(
        "SOL-USD".to_string(),
        GradientConfig::new(
            Color::Rgb(154, 69, 254), // PURUPLE
            Color::Rgb(87, 152, 203), // PURPLE - GREEN
            Color::Rgb(87, 152, 203), // PURPLE - GREEN
            Color::Rgb(21, 240, 150), // GREEN
            Color::Rgb(21, 240, 150), // GREEN
            Color::Rgb(87, 152, 203), // PURPLE - GREEN
            Color::Rgb(87, 152, 203), // PURPLE - GREEN
            Color::Rgb(154, 69, 254), // PURUPLE
        )
    ),]);
}

fn convert_timestamp_to_locale(ts: f64) -> String {
    let local: DateTime<Local> = Utc
        .timestamp_millis_opt(ts as i64)
        .unwrap()
        .with_timezone(&Local);
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn hilo(data: &Vec<WsMessage>) -> (f64, f64) {
    let mut hi = 0.0;
    let mut lo = 0.0;

    // wtf
    for ele in data {
        let l_hi = ele.high_24h.parse::<f64>().unwrap_or(0.0);
        let l_lo = ele.low_24h.parse::<f64>().unwrap_or(0.0);

        if l_hi > hi {
            hi = l_hi;
        }

        if l_lo > lo {
            lo = l_lo;
        }
    }

    (hi, lo)
}

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

                let t_changee = 60000.0;
                let p_changee = 1.0;

                match self.watching.len() {
                    1 => {
                        self.render_chart(
                            frame,
                            frame.area(),
                            self.watching[0].clone(),
                            t_changee,
                            p_changee,
                        );
                        return;
                    }
                    2 => {
                        let [top, bottom] =
                            Layout::vertical([Constraint::Fill(1); 2]).areas(frame.area());
                        self.render_chart(
                            frame,
                            top,
                            self.watching[0].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            bottom,
                            self.watching[1].clone(),
                            t_changee,
                            p_changee,
                        );
                    }
                    3 => {
                        let [left, right] =
                            Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
                        let [top, bottom] = Layout::vertical([Constraint::Fill(1); 2]).areas(left);

                        self.render_chart(
                            frame,
                            top,
                            self.watching[0].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            bottom,
                            self.watching[1].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            right,
                            self.watching[2].clone(),
                            t_changee,
                            p_changee,
                        );
                    }
                    4 => {
                        let [left, right] =
                            Layout::horizontal([Constraint::Fill(1); 2]).areas(frame.area());
                        let [l_top, l_bottom] =
                            Layout::vertical([Constraint::Fill(1); 2]).areas(left);
                        let [r_top, r_bottom] =
                            Layout::vertical([Constraint::Fill(1); 2]).areas(right);

                        self.render_chart(
                            frame,
                            l_top,
                            self.watching[0].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            l_bottom,
                            self.watching[1].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            r_top,
                            self.watching[2].clone(),
                            t_changee,
                            p_changee,
                        );
                        self.render_chart(
                            frame,
                            r_bottom,
                            self.watching[3].clone(),
                            t_changee,
                            p_changee,
                        );
                    }
                    _ => {}
                }
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

    fn render_chart(
        &self,
        frame: &mut Frame,
        area: Rect,
        coin: String,
        t_changee: f64,
        p_changee: f64,
    ) {
        // add filtering for coins

        let tmp_data = fff
            .lock()
            .clone()
            .iter()
            .filter(|f| f.product_id == coin)
            .map(|f| f.to_owned())
            .collect::<Vec<WsMessage>>();

        let data = tmp_data
            .iter()
            .map(|i| {
                let time = i.time.parse::<DateTime<chrono::Utc>>().unwrap();
                (
                    time.timestamp_millis() as f64,
                    i.price.parse::<f64>().unwrap_or(0.0),
                )
            })
            .collect::<Vec<(f64, f64)>>();

        let now = chrono::Local::now().timestamp_millis() as f64;
        let last = data.last().unwrap_or(&(0.0, 0.0));
        let price = last.1;

        // TIME AXIS
        let x_axis = Axis::default()
            .style(Color::White)
            .bounds([now - t_changee * 5.0, now + t_changee])
            .labels([
                convert_timestamp_to_locale(now - t_changee * 5.0).white(),
                convert_timestamp_to_locale(now + t_changee).white(),
            ]);

        let price_1per = price / 100.0;

        // PRICE AXIS
        let y_axis = Axis::default()
            .bounds([price_1per * 99.0, price_1per * 101.0])
            .labels([
                format!("{:.2}", price_1per * 98.0).red(),
                price.to_string().white(),
                format!("{:.2}", price_1per * 102.0).green(),
            ])
            .style(Color::White);

        /* let data = (1..1000)
        .map(|i: u64| (i as f64, i.pow(2) as f64))
        .collect::<Vec<(f64, f64)>>(); */

        let buys = tmp_data.iter().filter(|f| f.side == "buy").count();

        // If we have an overall surpluss of buys, we display it green to show the past 5k request
        // bias
        let color = if buys > tmp_data.len() / 2 {
            Color::Rgb(0, 255, 100)
        } else {
            Color::Rgb(255, 0, 100)
        };

        let title = format!("{} - {} - {}", coin, convert_timestamp_to_locale(now), fff.lock().len());

        let chart = Chart::new(vec![
            Dataset::default()
                .style(color)
                .marker(symbols::Marker::Braille)
                .data(&data),
        ])
        .x_axis(x_axis)
        .y_axis(y_axis);

        let widget = GradientWrapper::new(chart).title(title).gradient_colors(
            CRYPTO_COLOR_CODES
                .get(&coin)
                .unwrap_or(&GradientConfig::default())
                .clone(),
        );
        frame.render_widget(widget, area);
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
