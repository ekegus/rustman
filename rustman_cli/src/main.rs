use reqwest::Error;
use rustman_core::{Game, GameOutcome, GameState};
use serde_json::Value;
use std::io;

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

    let max_attempts: i8 = 5;

    let mut game = Game::new(secret_word, max_attempts);

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

fn fetch_data(url: &str) -> Result<String, Error> {
    let body = reqwest::blocking::get(url)?.text()?;

    return Ok(body);
}

fn parse_word_from_json(json_str: &str) -> Option<String> {
    let json_value: Result<Value, _> = serde_json::from_str(json_str);

    match json_value {
        Ok(value) => {
            if let Some(word) = value.as_array()?.get(0)?.as_str() {
                Some(word.to_string())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
