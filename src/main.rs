use rand::Rng;
use std::io;

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Won,
    Lost,
}

struct Game {
    attempts_left: i8,
    state: GameState,
    secret_word: String,
    guessed_letters: Vec<char>,
}

impl Game {
    fn new(secret_word: String, max_attempts: i8) -> Self {
        Game {
            attempts_left: max_attempts,
            state: GameState::InProgress,
            secret_word,
            guessed_letters: Vec::with_capacity(max_attempts as usize),
        }
    }

    fn has_won(&mut self) -> bool {
        self.secret_word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    fn has_lost(&mut self) -> bool {
        self.attempts_left == 0
    }

    fn get_guess(&mut self) -> String {
        println!("\n ========== \n");

        println!("Attempts left: {}", self.attempts_left);
        println!("Word: {}", self.get_display_word());
        println!(
            "Chars guessed: {}",
            self.guessed_letters.iter().collect::<String>()
        );

        let mut guess = String::from("");

        println!("Please make a guess...");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess.trim().to_string()
    }

    fn evaluate_guess(&mut self, letter: char) -> () {
        self.guessed_letters.push(letter);

        match self.secret_word.contains(letter) {
            true => {
                if self.has_won() == true {
                    self.state = GameState::Won;
                }
            }
            false => {
                self.attempts_left -= 1;
                if self.has_lost() {
                    self.state = GameState::Lost;
                }
            }
        }
    }

    fn get_display_word(&mut self) -> String {
        self.secret_word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    }

    fn check_winner(&mut self) -> () {
        match self.state {
            GameState::Won => println!(
                "Congratulations! You guessed the word: {}",
                self.secret_word
            ),
            GameState::Lost => println!(
                "Sorry, you ran out of attempts. The word was: {}",
                self.secret_word
            ),
            _ => (),
        }
    }
}

fn main() -> () {
    // TODO: Get the words from an API?
    let words = ["flower", "cola", "pirate"];
    let secret_word = select_random_item(&words).expect("Empty list!").to_string();
    let max_attempts: i8 = 5;

    let mut game = Game::new(secret_word, max_attempts);

    while game.state == GameState::InProgress {
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
