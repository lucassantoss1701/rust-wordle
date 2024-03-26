use entity::game::Game;
use std::io::{self, Write}; // Importe o trait Write para poder chamar flush()

mod entity;

fn main() {
    let mut game = Game::new_game();
    let mut num_attempts = 0;
    let max_attempts = 5;

    loop {
        println!("\n\n");
        println!("Chance {}/{}", num_attempts + 1, max_attempts);
        print!("Digite seu chute (5 letras): ");
        io::stdout().flush().expect("Falha ao exibir mensagem");

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
            for v in game.tries[game.number_of_tries - 1].get_letters() {
                print!("{}", v);
            }

            println!("\nVocê ganhou!");
            break;
        } else {
            num_attempts += 1;
            if num_attempts >= max_attempts {
                println!("Você excedeu o número máximo de tentativas.");
                println!("A palavra era: {:?}", game.word);
                break;
            }

            println!("");

            for v in game.tries[game.number_of_tries - 1].get_letters() {
                print!("{}", v);
            }

            println!("\n\n-----------------------");
            println!("Tente novamente!");
            println!("-----------------------");
        }
    }
}
