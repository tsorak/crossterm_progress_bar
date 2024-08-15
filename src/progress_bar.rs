use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal, ExecutableCommand,
};
use std::io::{stdout, Write};

mod alias;
mod chain;
mod style;

use alias::ColumnCount;
use style::Style;

#[derive(Debug, Clone)]
pub struct ProgressBar {
    value: usize,
    max_value: usize,
    width: Width,
    show_percent: bool,
    pub style: Style,
}

#[derive(Debug, Clone)]
pub enum Width {
    Stretch,
    Absolute(ColumnCount),
}

impl ProgressBar {
    pub fn new(max_value: usize) -> Self {
        Self {
            value: 0,
            max_value,
            width: Width::Stretch,
            show_percent: true,
            style: Style::default(),
        }
    }

    /// Creates a ProgressBar with x progress achieved
    pub fn new_at(progress: usize, max_value: usize) -> Self {
        if progress > max_value {
            core::panic!("progress may not be greater than max_value")
        }

        Self {
            value: progress,
            max_value,
            width: Width::Stretch,
            show_percent: true,
            style: Style::default(),
        }
    }

    /// Sets progress towards the max_value
    pub fn set_progress(&mut self, value: usize) -> &mut Self {
        if value > self.max_value {
            #[cfg(debug_assertions)]
            eprintln!("[crossterm_progress_bar] Provided value exceeds max_value");

            if self.value != self.max_value {
                self.value = self.max_value;
            }

            return self;
        }

        self.value = value;
        self
    }

    /// Total width the progressbar stretches over (terminal columns)
    pub fn set_width(mut self, width: impl Into<Width>) -> Self {
        self.with_width(width);
        self
    }

    pub fn set_show_percent(mut self, b: bool) -> Self {
        self.with_show_percent(b);
        self
    }

    /// Returns the bar in barebones form. As a String.
    ///
    /// No borders or percentage included.
    ///
    /// May error if terminal size can't be determined (Only if the Width::Stretch option is used).
    pub fn render_barebones_to_string(&self) -> Result<String, crate::Error> {
        let progress = self.value as f32 / self.max_value as f32;

        let width = {
            match self.width {
                Width::Stretch => terminal::size().map_err(crate::Error::Stretch)?.0 as usize,
                Width::Absolute(n) => n,
            }
        };

        let (filled_width, empty_width) = calculate_bar_sizing(self.show_percent, progress, width);

        Ok(generate_bar_string(&self.style, filled_width, empty_width))
    }

    /// Returns the bar in its final form. As a String.
    ///
    /// May error if terminal size can't be determined (Only if the Width::Stretch option is used).
    pub fn render_to_string(&self) -> Result<String, crate::Error> {
        let progress = self.value as f32 / self.max_value as f32;

        let width = {
            match self.width {
                Width::Stretch => terminal::size().map_err(crate::Error::Stretch)?.0 as usize,
                Width::Absolute(n) => n,
            }
        };

        let (filled_width, empty_width) = calculate_bar_sizing(self.show_percent, progress, width);

        let bar_barebones = generate_bar_string(&self.style, filled_width, empty_width);

        Ok(add_borders_and_percentage(
            self.show_percent,
            bar_barebones,
            progress,
        ))
    }

    /// Write the bar to stdout
    pub fn render(&self) -> Result<(), crate::Error> {
        let bar_component = self.render_to_string()?.stylize();

        // Print the progress bar
        let mut stdout = stdout();
        stdout.execute(cursor::SavePosition)?;
        stdout.execute(PrintStyledContent(bar_component))?;
        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;

        Ok(())
    }

    fn clear_line(&self) -> Result<(), std::io::Error> {
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(())
    }
}

fn calculate_bar_sizing(
    adjust_to_percent_section: bool,
    progress: f32,
    width: usize,
) -> (usize, usize) {
    if adjust_to_percent_section {
        let filled_width = (progress * (width - 10) as f32) as usize;
        let empty_width = width - 10 - filled_width;
        (filled_width, empty_width)
    } else {
        let filled_width = (progress * (width - 3) as f32) as usize;
        let empty_width = width - 3 - filled_width;
        (filled_width, empty_width)
    }
}

fn generate_bar_string(style: &Style, filled_width: usize, empty_width: usize) -> String {
    format!(
        "{}{}{}",
        format!("{}", style.fill_char).repeat(filled_width),
        style.arrow_char,
        format!("{}", style.empty_char).repeat(empty_width),
    )
}

fn add_borders_and_percentage(show_percent: bool, bar: String, progress: f32) -> String {
    if show_percent {
        format!("[{}] {:.1}%", bar, progress * 100.0)
    } else {
        format!("[{}]", bar)
    }
}

impl From<usize> for Width {
    fn from(value: usize) -> Self {
        Self::Absolute(value)
    }
}

/// Use this at your own risk :D (please dont)
impl From<&str> for Width {
    fn from(value: &str) -> Self {
        match value {
            "full" | "fill" | "stretch" => Self::Stretch,
            _ => core::panic!("Invalid ProgressBar Width specifier"),
        }
    }
}
