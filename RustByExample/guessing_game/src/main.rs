// Importing necessary modules from the Rust standard library
use std::io; // For handling input and output
use std::cmp::Ordering; // For comparing values
use rand::Rng; // For generating random numbers

fn main() {
    // Printing a welcome message to the console
    println!("Guess the number!");

    // Generating a random number between 1 and 100, inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Starting an infinite loop to allow repeated guessing
    loop {
        // Prompting the user to input their guess
        println!("Input a number to guess!");

        // Creating a mutable string to store the user's input
        let mut guess = String::new();

        // Reading the user's input from standard input and storing it in the 'guess' variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read input.");

        // Trimming any whitespace and attempting to parse the input as a 32-bit unsigned integer
        // If parsing fails, the program will panic with the message "Please type a number!"
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Comparing the user's guess with the secret number
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret number, print "Too small."
            Ordering::Less => println!("Too small."),
            // If the guess is equal to the secret number, print "Correct guess." and break out of the loop
            Ordering::Equal => {
                println!("Correct guess.");
                break;
            },
            // If the guess is greater than the secret number, print "Too large."
            Ordering::Greater => println!("Too large."),
        }
    }
}
