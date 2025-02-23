use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // Generate a random number in range [0:100]
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        // Using the io to read a line and if can't do the expect
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read you number!");

        // Here we can "change" the type of guess, if you give some string this gonna be converted to a number
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Handling invalid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type a number");
                continue;
            }
        };

        println!("You guessed the number {guess}");

        // Comparing the variables guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("So small"),
            Ordering::Equal => {
                println!("That's it");
                break;
            }
            Ordering::Greater => println!("So big"),
        }
    }
}
