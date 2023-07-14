use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number from 1 to 100!");
    
    let mut rng = rand::thread_rng();

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let parse_result : Result<i32, _> = guess.trim().parse();

        let guessed_number : i32;

        match parse_result {
            Ok(n) => {
                guessed_number = n;
                println!("parsed: {}", guessed_number);
            }
            Err(err) => {
                println!("failed: {}", err);
                continue;
            }
        }

        println!("You guessed: {guess}");

        let number : i32 = rng.gen_range(1..=100);

        println!("The number was: {number}");
        
        if guessed_number == number {
            println!("You won!");
        } else {
            println!("Try again!");
        }
    }
}
