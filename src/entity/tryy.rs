use super::letter::Letter;
use std::fmt;

pub struct Try {
    pub letters: Vec<Letter>,
}

impl Default for Try {
    fn default() -> Self {
        Try {
            letters: Vec::new(),
        }
    }
}

impl Try {
    pub fn get_letters(&self) -> &Vec<Letter> {
        &self.letters
    }

    pub fn get_letters_mut(&mut self) -> &mut Vec<Letter> {
        &mut self.letters
    }

    pub fn new() -> Try {
        Try {
            letters: Vec::new(),
        }
    }
}

impl fmt::Display for Try {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for letter in &self.letters {
            write!(f, "{}", letter)?;
        }
        Ok(())
    }
}
