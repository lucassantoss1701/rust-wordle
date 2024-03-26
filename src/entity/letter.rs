use super::color::Color;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Letter {
    color: Color,
    value: char,
}

impl Letter {
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn new(color: Color, value: char) -> Letter {
        Letter { color, value }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_letter = match self.color {
            Color::Black => format!("\x1b[30m{}\x1b[0m", self.value), // Black
            Color::Green => format!("\x1b[32m{}\x1b[0m", self.value), // Green
            Color::Yellow => format!("\x1b[33m{}\x1b[0m", self.value), // Yellow
        };
        write!(f, "{}", colored_letter)
    }
}
