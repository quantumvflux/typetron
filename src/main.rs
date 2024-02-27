use std::io::{self};
use std::time::Instant;

fn main() {
    println!("Welcome to TypeTron!");

    loop {
        println!("Press ENTER to start typing or CTRL+C to quit: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let start_time = Instant::now();
        let text = "The quick brown fox jumps over the lazy dog";
        println!("Type the following text:");
        println!("{}", text);

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let end_time = Instant::now();

        let elapsed_time = end_time.duration_since(start_time);
        let elapsed_seconds = elapsed_time.as_secs_f64();
        let words_per_minute =
            (text.split_whitespace().count() as f64) / { elapsed_seconds / 60.0 };

        let mut incorrect_indexes = Vec::new();

        for (index, (expected_char, user_char)) in text.chars().zip(user_input.chars()).enumerate()
        {
            if user_char != expected_char {
                incorrect_indexes.push(index)
            }
        }

        for (index, c) in user_input.chars().enumerate() {
            if incorrect_indexes.contains(&index) {
                print!("\x1b[31m{}\x1b[0m", c);
            } else {
                print!("{}", c);
            }
        }

        println!();

        println!("Time elapsed: {:.2} seconds", elapsed_seconds);
        println!("Words per minute: {:2}", words_per_minute);
    }

    println!("Thanks for using TypeTron!");
}
