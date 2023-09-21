use rand::Rng;
use std::io;

fn main() -> () {
    let mut lives_left = 3;
    let words = ["flower", "cola", "pirate"];

    let random_word = select_random_item(&words).expect("Empty list!");
    let word_length = random_word.len();
    let mut obfuscated_word: Vec<char> = std::iter::repeat('_').take(word_length).collect();

    let mut guess_chars: String = String::new();

    while lives_left > 0 {
        println!("\n ========== \n");

        println!("Lives: {}", lives_left);
        println!("Word: {}", obfuscated_word.iter().collect::<String>());
        println!("Chars guessed: {}", guess_chars);

        let mut guess = String::from("");

        println!("Please make a guess...");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess_chars.push_str(&format!("{} ", &guess.trim()).to_string());

        if guess.trim().len() > 1 {
            println!("Only 1 character is allowed");
            continue;
        }

        let success = random_word.contains(&guess.to_lowercase().trim());

        if success {
            let mut indexes = vec![];

            for (idx, char) in random_word.chars().enumerate() {
                if char.to_string() == guess.to_lowercase().trim() {
                    indexes.push(idx);
                }
            }

            for idx in indexes.iter() {
                let guessed_char = guess
                    .chars()
                    .next()
                    .expect("Failed to update obfuscated word.");
                println!("{}", guessed_char);
                obfuscated_word[*idx] = guessed_char;
            }

            if obfuscated_word.iter().all(|x| x != &'_') {
                println!("You won.");
                break;
            };

            continue;
        } else {
            lives_left -= 1;
        }
    }

    if lives_left == 0 {
        println!(
            "Game over. You are out of lives.\n The word was: {}",
            random_word
        );
    }
}

fn select_random_item<T>(list: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..list.len());

    return list.get(random_index);
}
