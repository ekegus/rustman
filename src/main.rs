use hangman_cli::{Game, GameState};
use rand::Rng;

fn main() -> () {
    // TODO: Get the words from an API?
    // https://developer.wordnik.com/
    // https://random-word-api.herokuapp.com/word

    let words = ["flower", "cola", "pirate"];
    let secret_word = select_random_item(&words).expect("Empty list!").to_string();
    let max_attempts: i8 = 5;

    let mut game = Game::new(secret_word, max_attempts);

    while *game.get_game_state() == GameState::InProgress {
        let guess = game.get_guess();

        if guess.len() > 1 {
            println!("Only 1 character is allowed");
            continue;
        }

        if let Some(letter) = guess.chars().next() {
            game.evaluate_guess(letter)
        }
    }

    game.check_winner();
}

fn select_random_item<T>(list: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..list.len());

    return list.get(random_index);
}
