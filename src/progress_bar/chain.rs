use crossterm::style::StyledContent;

use super::*;

impl ProgressBar {
    /// Whether to show progress percentage
    pub fn with_show_percent(&mut self, b: bool) -> &mut Self {
        self.show_percent = b;

        self.clear_line().unwrap();
        self.render().unwrap();

        self
    }

    /// Total width the progressbar stretches over (terminal columns)
    pub fn with_width(&mut self, width: impl Into<Width>) -> &mut Self {
        let width: Width = width.into();

        self.width = width;

        self
    }

    pub fn with_max_value(&mut self, value: usize) -> &mut Self {
        self.max_value = value;

        self
    }
}

impl Style {
    pub fn with_fill<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(StyledContent<char>) -> StyledContent<char> + Sized,
    {
        self.fill_char = f(self.fill_char);

        self
    }

    pub fn with_empty<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(StyledContent<char>) -> StyledContent<char> + Sized,
    {
        self.empty_char = f(self.empty_char);

        self
    }

    pub fn with_arrow<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(StyledContent<char>) -> StyledContent<char> + Sized,
    {
        self.arrow_char = f(self.arrow_char);

        self
    }
}
