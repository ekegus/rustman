use anyhow::anyhow;
use rustman_api::{fetch_data, parse_word_from_json};
use rustman_game::{Game, GameOutcome, GameState};
use std::io::{self, Write};

const MAX_ATTEMPTS: i8 = 5;
const URL: &str = "https:random-word-api.herokuapp.com/word";

fn get_secret_word() -> anyhow::Result<String> {
    let data = fetch_data(URL).map_err(|error| anyhow!("Failed to make API call: {}", error))?;

    let secret_word =
        parse_word_from_json(data.as_str()).ok_or(anyhow!("Failed to parse word from json"))?;

    Ok(secret_word)
}

fn get_user_input(user_instruction: &str) -> anyhow::Result<String> {
    println!("{}", user_instruction);

    let mut user_input = String::from("");

    io::stdout().flush()?;

    io::stdin().read_line(&mut user_input)?;

    Ok(user_input.trim().to_string())
}

fn play_round(game: &mut Game) -> ::anyhow::Result<()> {
    println!("Attempts left: {}", game.attempts_left);
    println!("Word: {}", game.get_display_word());
    println!(
        "Chars guessed: {}",
        &game.guessed_letters.iter().collect::<String>()
    );

    let guess = get_user_input("Please make a guess...")?;

    if guess.len() > 1 {
        return Ok(());
    }

    if let Some(letter) = guess.chars().next() {
        game.evaluate_guess(letter);
    }

    Ok(())
}

fn play_game(mut game: Game) -> anyhow::Result<()> {
    while *game.get_game_state() == GameState::InProgress {
        play_round(&mut game)?;
    }

    match game.evaluate_game_outcome() {
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
    return Ok(());
}

fn terminate_with_error(error: anyhow::Error) -> ! {
    eprintln!("{}", error);
    std::process::exit(1);
}

fn run() -> anyhow::Result<()> {
    let secret_word = get_secret_word()?;
    let game = Game::new(secret_word, MAX_ATTEMPTS);
    play_game(game)?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        terminate_with_error(error);
    }
}
