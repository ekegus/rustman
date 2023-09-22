use rand::Rng;
use std::io;

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Won,
    Lost,
}

struct Game {
    attempts_left: i32,
    state: GameState,
    secret_word: String,
}

fn main() -> () {
    let words = ["flower", "cola", "pirate"];

    let mut game = Game {
        attempts_left: 3,
        state: GameState::InProgress,
        secret_word: select_random_item(&words).expect("Empty list!").to_string(),
    };

    let mut guessed_letters: Vec<char> = vec![];

    while game.state == GameState::InProgress {
        println!("\n ========== \n");

        println!("Attempts left: {}", game.attempts_left);
        println!(
            "Word: {}",
            get_display_word(&game.secret_word, &guessed_letters)
        );
        println!(
            "Chars guessed: {}",
            guessed_letters.iter().collect::<String>()
        );

        let mut guess = String::from("");

        println!("Please make a guess...");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_string();

        if guess.len() > 1 {
            println!("Only 1 character is allowed");
            continue;
        }

        if let Some(letter) = guess.chars().next() {
            guessed_letters.push(letter);

            let success = game.secret_word.contains(&guess.to_lowercase().trim());

            if success {
                match game
                    .secret_word
                    .chars()
                    .all(|c| guessed_letters.contains(&c))
                {
                    true => game.state = GameState::Won,
                    false => continue,
                }
            } else {
                game.attempts_left -= 1;
                if game.attempts_left == 0 {
                    game.state = GameState::Lost;
                    break;
                }
            }
        }
    }
    match game.state {
        GameState::Won => println!(
            "Congratulations! You guessed the word: {}",
            game.secret_word
        ),
        GameState::Lost => println!(
            "Sorry, you ran out of attempts. The word was: {}",
            game.secret_word
        ),
        _ => (),
    }
}

fn select_random_item<T>(list: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..list.len());

    return list.get(random_index);
}

fn get_display_word(secret_word: &str, guessed_letters: &[char]) -> String {
    secret_word
        .chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
        .collect::<String>()
}
