use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand,
};
use std::io::{stdout, Write};

mod alias;
mod chain;

use alias::ColumnCount;

pub struct ProgressBar {
    value: usize,
    max_value: usize,
    width: Width,
    show_percent: bool,
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

        let mut stdout = stdout();

        // Calculate the filled width
        let (filled_width, empty_width) = if self.show_percent {
            let filled_width = (progress * (width - 10) as f32) as usize;
            let empty_width = width - 10 - filled_width;
            (filled_width, empty_width)
        } else {
            let filled_width = (progress * (width - 3) as f32) as usize;
            let empty_width = width - 3 - filled_width;
            (filled_width, empty_width)
        };

        // Create the progress bar string
        let bar = "=".repeat(filled_width) + ">" + &" ".repeat(empty_width);

        // Print the progress bar
        stdout.execute(cursor::SavePosition)?;
        if self.show_percent {
            stdout.execute(style::PrintStyledContent(
                format!("[{}] {:.1}%", bar, progress * 100.0).stylize(),
            ))?;
        } else {
            stdout.execute(style::PrintStyledContent(format!("[{}]", bar).stylize()))?;
        }
        stdout.execute(cursor::RestorePosition)?;
        stdout.flush()?;

        Ok(())
    }

    fn clear_line(&self) -> anyhow::Result<()> {
        stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine))?;
        Ok(())
    }
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
