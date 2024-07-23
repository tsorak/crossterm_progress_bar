use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal, ExecutableCommand,
};
use std::io::{stdout, Stdout, Write};

mod alias;
mod chain;
mod style;

use alias::ColumnCount;
use style::Style;

pub struct ProgressBar {
    value: usize,
    max_value: usize,
    width: Width,
    show_percent: bool,
    pub style: Style,
}

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

    /// Sets progress towards the max_value and rerenders the bar
    pub fn set_progress(&mut self, value: usize) -> anyhow::Result<()> {
        if value > self.max_value {
            #[cfg(debug_assertions)]
            eprintln!("[crossterm_progress_bar] Provided value exceeds max_value");

            if self.value != self.max_value {
                self.value = self.max_value;
                self.render()?;
            }

            return Ok(());
        }

        self.value = value;

        self.render()?;

        Ok(())
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

    fn render(&self) -> anyhow::Result<()> {
        let max_value = self.max_value;

        let progress = self.value as f32 / max_value as f32;

        let width = {
            match self.width {
                Width::Stretch => terminal::size()?.0 as usize,
                Width::Absolute(n) => n,
            }
        };

        // Calculate the filled width
        let (filled_width, empty_width) = calculate_bar_sizing(self.show_percent, progress, width);

        // Create the progress bar string
        let bar = generate_bar_string(&self.style, filled_width, empty_width);

        // Print the progress bar
        let mut stdout = stdout();
        stdout.execute(cursor::SavePosition)?;
        draw(&mut stdout, self.show_percent, bar, progress)?;
        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;

        Ok(())
    }

    fn clear_line(&self) -> anyhow::Result<()> {
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

fn draw(
    stdout: &mut Stdout,
    show_percent: bool,
    bar: String,
    progress: f32,
) -> anyhow::Result<&mut std::io::Stdout, std::io::Error> {
    let line = if show_percent {
        format!("[{}] {:.1}%", bar, progress * 100.0).stylize()
    } else {
        format!("[{}]", bar).stylize()
    };

    stdout.execute(PrintStyledContent(line.stylize()))
}

impl From<usize> for Width {
    fn from(value: usize) -> Self {
        Self::Absolute(value)
    }
}

impl From<&str> for Width {
    fn from(value: &str) -> Self {
        match value {
            "full" | "fill" | "stretch" => Self::Stretch,
            _ => core::panic!("Invalid ProgressBar Width specifier"),
        }
    }
}
