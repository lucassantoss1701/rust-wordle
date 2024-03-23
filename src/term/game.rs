#[derive(PartialEq, Clone)]
enum Color {
    Green,
    Yellow,
}

struct Try {
    letters: Vec<Letter>,
}

#[derive(Clone)]
struct Letter {
    color: Color,
    value: char,
}

pub struct Game {
    word: [char; 5],
    number_of_tries: usize,
    tries: [Try; 6],
}

impl Default for Try {
    fn default() -> Self {
        Try {
            letters: Vec::new(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            word: [' '; 5],
            number_of_tries: 0,
            tries: Default::default(),
        }
    }
}

impl Game {
    pub fn new_game(word: [char; 5]) -> Game {
        Game {
            word: word,
            number_of_tries: 0,
            tries: Default::default(),
        }
    }

    fn win(&self) -> bool {
        let last_try = &self.tries[self.number_of_tries - 1];
        last_try
            .letters
            .iter()
            .all(|letter| letter.color == Color::Green)
    }

    pub fn new_try(&mut self, guess: [char; 5]) -> bool {
        let mut letters: Vec<Letter> = Vec::new();

        for (i, &c) in guess.iter().enumerate() {
            if self.word[i] == c {
                letters.push(Letter {
                    color: Color::Green,
                    value: c,
                });
            } else {
                letters.push(Letter {
                    color: Color::Yellow,
                    value: c,
                });
            }
        }

        self.tries[self.number_of_tries] = Try {
            letters: letters.clone(),
        };
        self.number_of_tries += 1;

        for c in letters {
            match c.color {
                Color::Green => println!("Letter {} | Color Green", c.value),
                Color::Yellow => println!("Letter {} | Color Yellow", c.value),
            }
        }

        self.win()
    }
}
