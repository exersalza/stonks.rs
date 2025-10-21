//! Gradient border wrapper for ratatui widgets
//!
//! This module provides functionality to wrap any ratatui widget with a customizable
//! gradient border using rounded corners.

use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Color,
    text::{Line, Span},
    widgets::Widget,
};

/// Interpolates between two RGB colors based on a ratio
pub fn interpolate_color(start: Color, end: Color, ratio: f32) -> Color {
    match (start, end) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f32 + (r2 as f32 - r1 as f32) * ratio) as u8;
            let g = (g1 as f32 + (g2 as f32 - g1 as f32) * ratio) as u8;
            let b = (b1 as f32 + (b2 as f32 - b1 as f32) * ratio) as u8;
            Color::Rgb(r, g, b)
        }
        _ => start,
    }
}

fn set_c_fg(buf: &mut Buffer, x: u16, y: u16, fg: Option<Color>, ch: char) {
    if let Some(c) = buf.cell_mut(Position::new(x, y)) {
        c.set_char(ch);

        if let Some(col) = fg {
            c.set_fg(col);
        }
    }
}

/// Configuration for gradient colors on borders
#[derive(Clone, Debug)]
pub struct GradientConfig {
    /// This starts in the top left and goes to the right
    pub top_start: Color,
    pub top_end: Color,
    /// This starts at the top right corner and goes down to the bottom right corner
    pub right_start: Color,
    pub right_end: Color,
    /// This starts at the BOTTOM RIGHT corner and goes to the left corner
    pub bottom_start: Color,
    pub bottom_end: Color,
    /// This starts at the BOTTOM LEFT corner and goes up to the top left corner
    pub left_start: Color,
    pub left_end: Color,
}

impl Default for GradientConfig {
    fn default() -> Self {
        Self {
            top_start: Color::Rgb(255, 0, 0),
            top_end: Color::Rgb(0, 0, 255),
            right_start: Color::Rgb(0, 0, 255),
            right_end: Color::Rgb(0, 255, 0),
            bottom_start: Color::Rgb(0, 255, 0),
            bottom_end: Color::Rgb(255, 0, 255),
            left_start: Color::Rgb(255, 0, 255),
            left_end: Color::Rgb(255, 0, 0),
        }
    }
}

impl GradientConfig {
    /// Creates a new gradient configuration with default values
    pub fn new(
        top_start: Color,
        top_end: Color,
        right_start: Color,
        right_end: Color,
        bottom_start: Color,
        bottom_end: Color,
        left_start: Color,
        left_end: Color,
    ) -> Self {
        Self {
            top_start,
            top_end,
            right_start,
            right_end,
            bottom_start,
            bottom_end,
            left_start,
            left_end,
        }
    }

    pub fn new_1(c: Color) -> Self {
        Self {
            top_start: c,
            top_end: c,
            right_start: c,
            right_end: c,
            bottom_start: c,
            bottom_end: c,
            left_start: c,
            left_end: c,
        }
    }

    pub fn new_2(c: Color, c2: Color) -> Self {
        let inter = interpolate_color(c, c2, 0.5);
        Self {
            top_start: c,
            top_end: inter,
            right_start: inter,
            right_end: c2,
            bottom_start: c2,
            bottom_end: inter,
            left_start: inter,
            left_end: c,
        }
    }

    pub fn new_4(top_l: Color, top_r: Color, bot_r: Color, bot_l: Color) -> Self {
        Self {
            top_start: top_l,
            top_end: top_r,
            right_start: top_r,
            right_end: bot_r,
            bottom_start: bot_r,
            bottom_end: bot_l,
            left_start: bot_l,
            left_end: top_l,
        }
    }
}

/// Wrapper that renders any widget with a customizable gradient border
///
/// This struct wraps any ratatui widget and draws a gradient border around it
/// using rounded corners. The gradient colors are fully customizable.
///
/// # Examples
///
/// ```
/// use ratatui::widgets::{Chart, Dataset};
/// use crate::{GradientWrapper, GradientConfig};
///
/// let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);
/// let gradient_chart = GradientWrapper::new(chart)
///     .title("My Chart")
///     .gradient_colors(GradientConfig::ocean());
///
/// frame.render_widget(gradient_chart, area);
/// ```
pub struct GradientWrapper<'a, W> {
    widget: W,
    title: Option<Line<'a>>,
    gradient_config: GradientConfig,
}

impl<'a, W> GradientWrapper<'a, W> {
    /// Creates a new gradient wrapper around the given widget
    pub fn new(widget: W) -> Self {
        Self {
            widget,
            title: None,
            gradient_config: GradientConfig::default(),
        }
    }

    /// Sets the title to be displayed in the top border
    pub fn title<T: Into<Line<'a>>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the gradient configuration for the border colors
    pub fn gradient_colors(mut self, config: GradientConfig) -> Self {
        self.gradient_config = config;
        self
    }

    /// Draws the gradient border around the given area
    pub fn draw_gradient_border(&self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        let config = &self.gradient_config;

        // Calculate corner colors by blending horizontal and vertical gradients
        let top_left_color = config.top_start; // Start of both gradients

        let top_right_color = interpolate_color(
            config.top_start,
            config.top_end,
            1.0, // End of horizontal gradient
        );

        let bottom_left_color = interpolate_color(
            config.left_start,
            config.left_start,
            1.0, // End of vertical gradient
        );

        let bottom_right_color = interpolate_color(
            interpolate_color(config.right_start, config.right_end, 1.0),
            interpolate_color(config.right_start, config.right_end, 1.0),
            0.5, // Blend both end colors
        );

        // Draw ROUNDED corners WITH colors (using Unicode rounded corner characters)
        [
            (area.left(), area.top(), '╭', top_left_color),
            (area.right() - 1, area.top(), '╮', top_right_color),
            (area.left(), area.bottom() - 1, '╰', bottom_left_color),
            (area.right() - 1, area.bottom() - 1, '╯', bottom_right_color),
        ]
        .into_iter()
        .for_each(|(x, y, ch, fg)| {
            set_c_fg(buf, x, y, Some(fg), ch);
        });

        // Draw top and bottom borders with horizontal gradient
        for x in area.left() + 1..area.right() - 1 {
            let ratio = (x - area.left() - 1) as f32 / (area.width - 2) as f32;
            let color = interpolate_color(config.top_start, config.top_end, ratio);
            let b_color =
                interpolate_color(config.bottom_start, config.bottom_end, (ratio - 1.0).abs());

            set_c_fg(buf, x, area.top(), Some(color), '─');
            set_c_fg(buf, x, area.bottom() - 1, Some(b_color), '─');
        }

        // Draw left and right borders with vertical gradient
        for y in area.top() + 1..area.bottom() - 1 {
            let ratio = (y - area.top() - 1) as f32 / (area.height - 2) as f32;
            let r_color = interpolate_color(config.right_start, config.right_end, ratio);
            let color = interpolate_color(config.left_start, config.left_end, (ratio - 1.0).abs());

            set_c_fg(buf, area.left(), y, Some(color), '│');
            set_c_fg(buf, area.right() - 1, y, Some(r_color), '│');
        }

        // Draw title if provided
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.width() as u16 + 2)) / 2;
            if title_x < area.right() - 1 {
                set_c_fg(buf, title_x, area.top(), None, '┤');

                buf.set_line(
                    title_x + 1,
                    area.top(),
                    title,
                    title.width() as u16,
                );

                set_c_fg(buf, title_x + 1 + title.width() as u16, area.top(), None, '├');
            }
        }
    }
}

impl<W: Widget> Widget for GradientWrapper<'_, W> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Draw gradient border first
        self.draw_gradient_border(area, buf);

        // Calculate inner area (inside the border)
        let inner_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        // Render the wrapped widget in the inner area
        self.widget.render(inner_area, buf);
    }
}
