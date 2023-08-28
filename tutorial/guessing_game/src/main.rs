use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    // 1. Generate a random number
    // 2. Take input from user = guess
    // 3. Compare the guess and the random number

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("The random number is {random_number}");

    loop {
        let mut guess = String::new();
        println!("Guess a number between 1 and 100");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("The guess is lower than expected."),
            Ordering::Equal => {
                println!("You guessed the right number!");
                break;
            },
            Ordering::Greater => println!("The guess is higher than expected"),
        }
    }
}
