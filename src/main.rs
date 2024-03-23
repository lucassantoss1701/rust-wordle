use term::game::Game;
use term::words::new_random_word;

mod term;

fn main() {
    let word = new_random_word();

    for c in word {
        println!("{}", c);
    }

    let mut game = Game::new_game(word);

    loop {
        println!("Digite seu chute (5 letras):");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess: Vec<char> = input.trim().chars().collect();
        if guess.len() != 5 {
            println!("Por favor, insira 5 letras.");
            continue;
        }

        let guess_array: [char; 5] = [guess[0], guess[1], guess[2], guess[3], guess[4]];

        if game.new_try(guess_array) {
            println!("Você ganhou!");
            break;
        } else {
            println!("Tente novamente!");
        }
    }
}
