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
