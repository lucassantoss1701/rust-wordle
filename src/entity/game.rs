use super::color::Color;
use super::letter::Letter;
use super::tryy::Try;
use super::words::new_random_word;
use std::fmt;

pub struct Game {
    pub word: [char; 5],
    pub number_of_tries: usize,
    pub tries: Vec<Try>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for try_item in &self.tries[..self.number_of_tries] {
            writeln!(f, "{}", try_item)?;
        }
        Ok(())
    }
}

impl Game {
    pub fn new_game() -> Game {
        let word = new_random_word();
        // for c in word {
        //     print!("{}", c);
        // }
        println!("");

        Game {
            word: word,
            number_of_tries: 0,
            tries: Default::default(),
        }
    }

    fn win(&self) -> bool {
        let last_try = &self.tries[self.number_of_tries - 1];

        last_try
            .get_letters()
            .iter()
            .all(|letter| letter.get_color() == Color::Green)
    }

    pub fn new_try(&mut self, guess: [char; 5]) -> bool {
        let mut actual_try = Try::new();

        for (guess_i, &guess_letter) in guess.iter().enumerate() {
            let mut puxed = false;

            for (_, &word_letter) in self.word.iter().enumerate() {
                if self.word[guess_i] == guess[guess_i] {
                    actual_try
                        .get_letters_mut()
                        .push(Letter::new(Color::Green, guess_letter));

                    puxed = true;

                    break;
                }

                if guess_letter == word_letter {
                    actual_try
                        .get_letters_mut()
                        .push(Letter::new(Color::Yellow, guess_letter));
                    puxed = true;

                    break;
                }
            }

            if !puxed {
                actual_try
                    .get_letters_mut()
                    .push(Letter::new(Color::Black, guess_letter));
            }
        }

        self.tries.push(actual_try);

        for v in self.tries.iter().take(self.number_of_tries) {
            println!("{}", v);
        }

        self.number_of_tries = self.number_of_tries + 1;

        self.win()
    }
}
