use std::io;

#[derive(PartialEq)]
pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub enum GameOutcome {
    Winner(String),
    Loser(String),
    InProgress,
}

pub struct Game {
    attempts_left: i8,
    state: GameState,
    secret_word: String,
    guessed_letters: Vec<char>,
}

impl Game {
    pub fn new(secret_word: String, max_attempts: i8) -> Self {
        Game {
            attempts_left: max_attempts,
            state: GameState::InProgress,
            secret_word,
            guessed_letters: Vec::with_capacity(max_attempts as usize),
        }
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.state
    }

    pub fn has_won(&mut self) -> bool {
        self.secret_word
            .chars()
            .all(|c| self.guessed_letters.contains(&c))
    }

    pub fn has_lost(&mut self) -> bool {
        self.attempts_left == 0
    }

    pub fn get_guess(&mut self) -> String {
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

    pub fn evaluate_guess(&mut self, letter: char) -> () {
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

    pub fn get_display_word(&mut self) -> String {
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

    pub fn check_winner(&mut self) -> GameOutcome {
        match self.state {
            GameState::Won => GameOutcome::Winner(self.secret_word.clone()),
            GameState::Lost => GameOutcome::Loser(self.secret_word.clone()),
            _ => GameOutcome::InProgress,
        }
    }
}
