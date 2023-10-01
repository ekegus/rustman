use anyhow::{anyhow, Result};
use rustman_api::{fetch_data, parse_word_from_json};
use rustman_game::{Game, GameOutcome, GameState};
use std::io;

const MAX_ATTEMPTS: i8 = 5;
const URL: &str = "https:random-word-api.herokuapp.com/word";

fn get_secret_word() -> Result<String> {
    let data = fetch_data(URL).map_err(|error| anyhow!("Failed to make API call: {}", error))?;

    let secret_word =
        parse_word_from_json(data.as_str()).ok_or(anyhow!("Failed to parse word from json"))?;

    Ok(secret_word)
}

fn get_user_input(user_instruction: &str) -> String {
    let mut user_input = String::from("");

    println!("{}", user_instruction);

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    user_input.trim().to_string()
}

fn play_game(mut game: Game) -> Game {
    while *game.get_game_state() == GameState::InProgress {
        println!("Attempts left: {}", game.attempts_left);
        println!("Word: {}", game.get_display_word());
        println!(
            "Chars guessed: {}",
            &game.guessed_letters.iter().collect::<String>()
        );

        let guess = game.get_guess(get_user_input);

        if guess.len() > 1 {
            continue;
        }

        if let Some(letter) = guess.chars().next() {
            game.evaluate_guess(letter)
        }
    }

    return game;
}

fn main() -> () {
    match get_secret_word() {
        Ok(secret_word) => {
            let game = Game::new(secret_word, MAX_ATTEMPTS);

            let mut game = play_game(game);

            let game_outcome = game.evaluate_game_outcome();

            match game_outcome {
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
        Err(error) => eprintln!("{}", error),
    }
}
