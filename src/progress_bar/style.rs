use crossterm::style::{StyledContent, Stylize};

#[derive(Debug, Clone)]
pub struct Style {
    pub fill_char: StyledContent<char>,
    pub empty_char: StyledContent<char>,
    pub arrow_char: StyledContent<char>,
}

impl Default for Style {
    fn default() -> Self {
        Self::new('=', ' ', '>')
    }
}

impl Style {
    pub fn new(fill_char: char, empty_char: char, arrow_char: char) -> Self {
        Self {
            fill_char: fill_char.stylize(),
            empty_char: empty_char.stylize(),
            arrow_char: arrow_char.stylize(),
        }
    }

    pub fn new_stylized(
        fill_char: StyledContent<char>,
        empty_char: StyledContent<char>,
        arrow_char: StyledContent<char>,
    ) -> Self {
        Self {
            fill_char,
            empty_char,
            arrow_char,
        }
    }
}
