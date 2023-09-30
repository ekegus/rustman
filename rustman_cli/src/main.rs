use rustman_api::{fetch_data, parse_word_from_json};
use rustman_game::{Game, GameOutcome, GameState};
use std::io;

const MAX_ATTEMPTS: i8 = 5;

fn main() -> () {
    let url = "https:random-word-api.herokuapp.com/word";

    let data = match fetch_data(url) {
        Ok(data) => data,
        Err(error) => panic!("Failed to make API call: {}", error),
    };

    let secret_word = match parse_word_from_json(data.as_str()) {
        Some(word) => word,
        None => panic!("Failed to parse word from json."),
    };

    let mut game = Game::new(secret_word, MAX_ATTEMPTS);

    let handler = |input: &str| -> String {
        let mut guess = String::from("");

        println!("{}", input);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        return guess;
    };

    while *game.get_game_state() == GameState::InProgress {
        println!("Attempts left: {}", game.attempts_left);
        println!("Word: {}", game.get_display_word());
        println!(
            "Chars guessed: {}",
            &game.guessed_letters.iter().collect::<String>()
        );

        let guess = game.get_guess(handler);

        if guess.len() > 1 {
            continue;
        }

        if let Some(letter) = guess.chars().next() {
            game.evaluate_guess(letter)
        }
    }

    let outcome = game.check_winner();

    match outcome {
        GameOutcome::Winner(secret_word) => {
            println!("Congratulations! You guessed the word: {}", secret_word)
        }
        GameOutcome::Loser(secret_word) => {
            println!(
                "Sorry, you ran out of attempts. The word was: {}",
                secret_word
            )
        }
        GameOutcome::InProgress => (),
    }
}
