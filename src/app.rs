use std::{
    collections::HashMap,
    fmt::Display,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
    usize,
};

use crate::{
    events::{AppEvent, Event, EventHandler},
    gradient_widget::{GradientConfig, GradientWrapper},
    memes::{MEMES, XorShift32},
    sockets::{WsMessage, ws_messages},
    utils::CURRENCIES,
};

use chrono::{DateTime, Local, TimeZone, Utc};
use lazy_static::lazy_static;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols,
    text::{Line, Text},
    widgets::{Axis, Chart, Dataset, Paragraph},
};
use ringbuffer::RingBuffer;

use crate::utils::CRYPTO_COLOR_CODES;

pub const UPPER_COLOR_BOUND: u8 = 255;
pub const LOWER_COLOR_BOUND: u8 = 150;
/// This should be easily subtractable/ addable to the two values above be. Otherwise the type goes
/// out of bounds and the program panics. So if the bounds are 150-255 you should use either 1 or 5
/// etc.
pub const CHANGE_COLOR_BY: u8 = 5;

lazy_static! {
    static ref WATCHING_AMOUNT: Arc<i32> = Arc::new(0);
}

fn convert_timestamp_to_locale(ts: f64) -> String {
    let local: DateTime<Local> = Utc
        .timestamp_millis_opt(ts as i64)
        .unwrap()
        .with_timezone(&Local);
    local.format("%Y-%m-%d %H:%M:%S").to_string()
}

const BODY_MIN_H: i32 = 10;
const BODY_MIN_W: i32 = 46;

pub enum WindowType {
    Master,
    Splace,
}

fn calc_body_layout(area: Rect, amount: usize, window_type: WindowType) -> Vec<Rect> {
    if amount == 1 {
        return Layout::vertical([Constraint::Percentage(100)])
            .areas::<1>(area)
            .to_vec();
    }

    match window_type {
        // WindowType::Master => todo!(),
        WindowType::Splace => {
            // flooring here so we can get the max fitable without any rendering problems
            let w_max = (area.width as f32 / BODY_MIN_W as f32)
                .floor()
                .min(amount as f32);
            let h_max = ((area.height as f32 / BODY_MIN_H as f32).floor()).min(amount as f32);

            let can_fit = h_max * w_max;

            // check if we can have an equal amount on the horizontal and vertical
            let [w_act, h_act] = if (amount as f64).sqrt() == (amount as f64).sqrt().floor() {
                [(amount as f64).sqrt(); 2]

            // else do some other math i dont understand anymore
            } else {
                let w_act = (w_max - amount as f32 + amount as f32).floor();
                let h_act = (amount as f32 / w_act).ceil();

                [w_act as f64, h_act as f64]
            };

            let mut filled_spots = 0.0;
            let mut ret_rects = vec![];

            let main_verti =
                Layout::vertical(vec![Constraint::Fill(1); h_act as usize]).split(area);

            for i in main_verti.iter() {
                if (amount as f64) - filled_spots == 1.0 {
                    ret_rects.push(
                        Layout::horizontal(vec![Constraint::Fill(1)])
                            .split(*i)
                            .to_vec(),
                    );
                    continue;
                }

                ret_rects.push(
                    Layout::horizontal(vec![Constraint::Fill(1); w_act as usize])
                        .split(*i)
                        .to_vec(),
                );
                filled_spots += w_act;
            }

            ret_rects.iter().map(|i| i.to_owned()).flatten().collect()
        }
        WindowType::Master => {
            let [left, right] =
                Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)]).areas(area);

            vec![left, right]
        }
    }
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

    /// Price mulitplier
    price_mult: HashMap<String, f64>,

    /// The current selected chart/ window
    active_window: i32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            watching: vec!["SOL-USD".to_string()],
            running: true,
            events: EventHandler::new(),
            start_time: Self::now(),
            border_animation: true,
            color: 255,
            color_add: false,
            price_mult: HashMap::from([("SOL-USD".to_string(), 0.5)]),
            active_window: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(watching: Option<Vec<String>>) -> Self {
        match watching {
            Some(v) => {
                let prepd = v
                    .iter()
                    .map(|f| (f.to_owned(), 0.5f64))
                    .collect::<Vec<(String, f64)>>();

                Self {
                    watching: v,
                    price_mult: prepd.into_iter().collect(),
                    ..Default::default()
                }
            }
            _ => Self::default(),
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards???")
            .as_secs()
    }

    fn get_coin_mult<T: Display>(&self, coin: T) -> f64 {
        self.price_mult
            .get(&coin.to_string())
            .unwrap_or(&1.0)
            .clone()
    }

    fn get_coin_mult_mut<T: Display>(&mut self, coin: T) -> &mut f64 {
        // We can unwrap here, as there should always be the coin gettable when we call this. This
        // should be gurranteed as we dont remove anything anywhere, if it breaks i gotta come back
        // tho
        self.price_mult.get_mut(&coin.to_string()).unwrap()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        let mut rng = XorShift32::new(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),
        );

        let (top_text, bottom_text) = MEMES
            .get(rng.gen_range(MEMES.len() as usize) as usize)
            .unwrap_or(&("hellol", "byel"));

        while self.running {
            terminal.draw(|frame| {
                if self.watching.len() == 0 {
                    frame.render_widget(
                        Text::from("You dont have any Coins selected").centered(),
                        frame.area(),
                    );
                    return;
                }
                // TODO: add layouts for different screen sizes and for the amount of chains to
                // watch
                let [top, body, bottom] = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Fill(1),
                    Constraint::Length(1),
                ])
                .areas(frame.area());

                let now =
                    convert_timestamp_to_locale(chrono::Local::now().timestamp_millis() as f64);

                frame.render_widget(
                    Paragraph::new(format!("{top_text}\n{}", now)).centered(),
                    top,
                );
                frame.render_widget(Line::from(*bottom_text).centered(), bottom);

                let layout: Vec<Rect> =
                    calc_body_layout(body, self.watching.len(), WindowType::Splace);

                for (i, v) in layout.iter().enumerate() {
                    if i >= self.watching.len() {
                        continue;
                    }
                    self.render_chart(frame, v.to_owned(), self.watching[i].clone(), 60000.0);
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
                    AppEvent::IncMult(fine) => {
                        let v = self
                            .get_coin_mult_mut(self.watching[self.active_window as usize].clone());
                        if fine {
                            *v += 0.01;
                        } else {
                            *v += 0.1;
                        }
                    }
                    AppEvent::DecMult(fine) => {
                        let v = self
                            .get_coin_mult_mut(self.watching[self.active_window as usize].clone());
                        if fine {
                            *v -= 0.01;
                        } else {
                            *v -= 0.1;
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }

    fn render_chart(&self, frame: &mut Frame, area: Rect, coin: String, t_changee: f64) {
        // add filtering for coins

        let tmp_data = match ws_messages.lock().clone().get(&coin) {
            Some(v) => v
                .iter()
                .filter(|f| f.product_id == coin)
                .map(|f| f.to_owned())
                .collect::<Vec<WsMessage>>(),
            None => return,
        };

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

        //                                                                  TIME AXIS
        let x_axis = Axis::default()
            .style(Color::White)
            .bounds([now - t_changee * 5.0, now + t_changee])
            .labels([
                convert_timestamp_to_locale(now - t_changee * 5.0).white(),
                convert_timestamp_to_locale(now + t_changee).white(),
            ]);

        let price_1per = price / 100.0;

        let hi = price_1per * (100.0 + self.get_coin_mult(&coin));
        let lo = price_1per * (100.0 - self.get_coin_mult(&coin));

        /* let (hi, lo) = match tmp_data.last() {
            Some(v) => (
                v.high_24h.parse::<f64>().unwrap_or(0.0),
                v.low_24h.parse::<f64>().unwrap_or(0.0),
            ),
            None => return,
        }; */

        let crc = match coin.split('-').collect::<Vec<&str>>()[1] {
            "EUR" => CURRENCIES[1],
            _ => CURRENCIES[0], // default to $
        };

        //                                                                  PRICE AXIS
        let y_axis = Axis::default()
            .bounds([lo, hi])
            .labels([
                format!("{crc}{:.2}{:.2}", lo, lo - price).red(),
                format!("{crc}{price}").white(),
                format!("{crc}{:.2}+{:.2}", hi, hi - price).green(),
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

        let title = format!("{} - {}", coin, tmp_data.len());

        let chart = Chart::new(vec![
            Dataset::default()
                .style(color)
                .marker(symbols::Marker::Braille)
                .data(&data),
        ])
        .x_axis(x_axis)
        .y_axis(y_axis);

        let c = coin.split('-').collect::<Vec<&str>>()[0];
        let widget = GradientWrapper::new(chart).title(title).gradient_colors(
            CRYPTO_COLOR_CODES
                .get(c)
                .unwrap_or(&GradientConfig::default())
                .clone(),
        );
        frame.render_widget(widget, area);
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        let is_shift = key_event.modifiers == KeyModifiers::SHIFT;
        let is_ctrl = key_event.modifiers == KeyModifiers::CONTROL;
        let is_alt = key_event.modifiers == KeyModifiers::ALT;

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if is_ctrl => self.events.send(AppEvent::Quit),
            KeyCode::Up => self.events.send(AppEvent::IncMult(is_shift)),
            KeyCode::Down => self.events.send(AppEvent::DecMult(is_shift)),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {}

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
